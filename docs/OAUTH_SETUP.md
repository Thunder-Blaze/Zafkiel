# OAuth Setup Guide

This guide will help you set up AniList OAuth authentication for Zafkiel.

## Prerequisites

- An AniList account (create one at https://anilist.co/signup if you don't have one)

## Step 1: Create an OAuth Application

1. Go to https://anilist.co/settings/developer
2. Click **"Create New Client"**
3. Fill in the application details:
   - **Name**: `Zafkiel` (or any name you prefer)
   - **Redirect URL**: `http://localhost:57575/auth/callback`
4. Click **"Save"**
5. You'll receive:
   - **Client ID** (a long string of numbers)
   - **Client Secret** (click "Show Secret" to reveal it)

## Step 2: Configure Environment Variables

1. Copy the `.env.example` file to `.env`:

   ```bash
   cp .env.example .env
   ```

2. Open `.env` and replace the placeholder values:

   ```env
   ANILIST_CLIENT_ID=1234567890  # Your actual Client ID
   ANILIST_CLIENT_SECRET=abcdef123456  # Your actual Client Secret
   ```

3. Save the file

## Step 3: Restart the Application

If the app is already running, restart it to load the new environment variables:

```bash
# Stop the current dev server (Ctrl+C)
# Start it again
bun dev
```

## Step 4: Test Authentication

1. Open the app (it should redirect you to `/login`)
2. Click **"Sign in with AniList"**
3. Your default browser should open with AniList's authorization page
4. Click **"Authorize"** to grant access
5. You'll be redirected back to the app automatically
6. The app will show your profile on the home page

## Troubleshooting

### "ANILIST_CLIENT_ID not found in environment"

- Make sure you created the `.env` file in the project root
- Make sure the file is named exactly `.env` (not `.env.txt` or similar)
- Restart the development server after creating/modifying `.env`

### "Failed to login" immediately after clicking sign in

- Check that your `.env` file has valid credentials
- Make sure there are no extra spaces or quotes around the values
- Verify the credentials in the AniList developer settings

### Browser doesn't open

- The app will fall back to opening a webview window
- Check the browser console and backend logs for errors
- Make sure port 57575 is not blocked by a firewall

### "OAuth timeout - no callback received"

- Make sure the redirect URL in your AniList OAuth app is exactly: `http://localhost:57575/auth/callback`
- Check that no firewall is blocking port 57575
- Try authorizing again - the server waits for 5 minutes

### Token not persisting between restarts

- Check that `~/.config/zafkiel/` directory is writable
- Look for errors in the backend logs related to config saving

## Security Notes

⚠️ **Never commit your `.env` file to version control!**

The `.env` file is already in `.gitignore`, but make sure you don't accidentally commit it.

Your credentials should remain private:

- Don't share your Client Secret with anyone
- Don't commit it to public repositories
- Don't include it in screenshots or logs

## Advanced Configuration

### Custom Callback Port

The default callback port is `57575`. If you need to use a different port:

1. Update the OAuth app settings in AniList with the new port
2. The app will automatically try different ports if 57575 is unavailable
3. Make sure the redirect URL matches: `http://localhost:{PORT}/auth/callback`

### Multiple Environments

You can create different OAuth apps for development, staging, and production:

- Development: `.env.development`
- Production: `.env.production`

Use different Client IDs and Secrets for each environment.

## Getting Help

If you encounter issues:

1. Check the backend logs in the terminal
2. Check the browser console for errors
3. Verify your OAuth app settings on AniList
4. Make sure your `.env` file is correctly formatted
5. Try deleting `~/.config/zafkiel/config.ron` to reset authentication

## Resources

- [AniList API Documentation](https://anilist.gitbook.io/anilist-apiv2-docs/)
- [OAuth 2.0 Specification](https://oauth.net/2/)
- [AniList Developer Settings](https://anilist.co/settings/developer)
