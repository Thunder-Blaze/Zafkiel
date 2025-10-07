# OAuth Flow Fix - Channel Timing Issue

## Problem

The authentication was stuck on "Connecting..." even though the callback was received. The logs showed:

```
[Auth] Received callback connection
[Auth] Extracted authorization code
```

But the frontend never got the token.

## Root Cause

**Channel Timing Mismatch:**

The original flow had a race condition:

1. `start_oauth_flow` → Starts callback server
2. `open_auth_browser` → User authorizes
3. **Callback received** → Server tries to send through channel
4. `wait_for_oauth_callback` → Creates NEW channel (too late!)

The channel was being created AFTER the callback had already been received, so the code was never received on the frontend side.

## Solution

### Changed Flow:

1. `start_oauth_flow`:
   - Creates channel (sender + receiver)
   - Stores sender in `auth_state.pending_auth` (for callback handler)
   - Stores receiver in `auth_state.pending_receiver` (for wait command)
   - Starts callback server
   - Returns auth URL

2. `open_auth_browser`:
   - Opens browser (unchanged)

3. **Callback received**:
   - Handler finds sender in `auth_state.pending_auth`
   - Sends code through channel
   - Server stops after one connection

4. `wait_for_oauth_callback`:
   - Retrieves receiver from `auth_state.pending_receiver`
   - Waits for code (already sent or will be sent)
   - Exchanges code for token
   - Saves token and updates service

### Code Changes

#### `auth.rs`

```rust
pub struct AuthState {
    pub pending_auth: Arc<Mutex<Option<oneshot::Sender<...>>>>,
    pub pending_receiver: Arc<Mutex<Option<oneshot::Receiver<...>>>>,  // NEW
}
```

#### `auth_commands.rs`

**`start_oauth_flow`:**

```rust
// Create channel
let (tx, rx) = tokio::sync::oneshot::channel();

// Store sender for callback handler
{
    let mut pending = auth_state.pending_auth.lock().await;
    *pending = Some(tx);
}

// Store receiver for wait_for_oauth_callback
{
    let mut receiver = auth_state.pending_receiver.lock().await;
    *receiver = Some(rx);
}

// ... start server, return auth URL
```

**`wait_for_oauth_callback`:**

```rust
// Get the receiver that was created in start_oauth_flow
let rx = {
    let mut receiver = auth_state.pending_receiver.lock().await;
    receiver.take().ok_or("No pending OAuth flow")?
};

// Wait for callback
let code = tokio::time::timeout(Duration::from_secs(300), rx).await...;
```

## Server Shutdown

The callback server automatically stops after handling one connection:

```rust
// In start_callback_server
tokio::task::spawn(async move {
    // Wait for exactly ONE connection
    match listener.accept() {
        Ok((mut stream, _)) => {
            handle_callback_request(&mut stream, auth_state).await;
            // Task ends here, server stops
        }
    }
});
```

No cleanup needed - the task and listener are dropped after the callback.

## Testing

1. Click "Sign in with AniList"
2. Browser opens with authorization page
3. Click "Authorize"
4. Browser shows success page
5. App automatically logs in and shows profile ✅

## Error Handling

### Better Error Messages

- Missing .env: `"ANILIST_CLIENT_ID not found in environment. Make sure .env file exists with ANILIST_CLIENT_ID and ANILIST_CLIENT_SECRET"`
- No pending flow: `"No pending OAuth flow. Call start_oauth_flow first."`
- Timeout: `"OAuth timeout - no callback received within 5 minutes"`

## Logs

Successful flow logs:

```
[Auth Command] Starting OAuth flow
[Auth Command] Using port 57575 for callback
[Auth] Starting callback server on port 57575
[Auth] Callback server listening on port 57575
[Auth Command] Authorization URL generated
[Auth Command] Opening authorization URL
[Auth Command] Waiting for OAuth callback
[Auth] Received callback connection
[Auth] Extracted authorization code
[Auth Command] Received authorization code, exchanging for token
[Auth Command] Token saved to config
[Auth Command] OAuth flow completed successfully
```

## Result

✅ OAuth flow now completes successfully
✅ Token saved to config
✅ Service updated with token
✅ Callback server stops after receiving token
✅ User automatically redirected to home page with profile
