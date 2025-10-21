use aes_gcm::{
    aead::{Aead, KeyInit}, Aes256Gcm, Nonce
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use rand::{rngs::OsRng, RngCore};
use thiserror::Error;

/// Encryption-related errors
#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Failed to encrypt data: {0}")]
    EncryptionFailed(String),

    #[error("Failed to decrypt data: {0}")]
    DecryptionFailed(String),

    #[error("Invalid key format: {0}")]
    InvalidKey(String),

    #[error("Invalid encrypted data format")]
    InvalidFormat,
}

/// Generate a new random 256-bit encryption key
pub fn generate_key() -> String {
    let mut key_bytes = [0u8; 32]; // 256 bits
    OsRng.fill_bytes(&mut key_bytes);
    BASE64.encode(key_bytes)
}

/// Encrypt data using AES-256-GCM
pub fn encrypt(data: &str, key_base64: &str) -> Result<String, EncryptionError> {
    // Decode the base64 key
    let key_bytes = BASE64
        .decode(key_base64)
        .map_err(|e| EncryptionError::InvalidKey(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(EncryptionError::InvalidKey(format!(
            "Expected 32 bytes, got {}",
            key_bytes.len()
        )));
    }

		let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();

    // Generate a random nonce (12 bytes for GCM)
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from(nonce_bytes);

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    // Combine nonce + ciphertext and encode as base64
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(result))
}

/// Decrypt data using AES-256-GCM
pub fn decrypt(encrypted_base64: &str, key_base64: &str) -> Result<String, EncryptionError> {
    // Decode the base64 key
    let key_bytes = BASE64
        .decode(key_base64)
        .map_err(|e| EncryptionError::InvalidKey(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(EncryptionError::InvalidKey(format!(
            "Expected 32 bytes, got {}",
            key_bytes.len()
        )));
    }

    // Decode the encrypted data
    let encrypted_data = BASE64
        .decode(encrypted_base64)
        .map_err(|_| EncryptionError::InvalidFormat)?;

    if encrypted_data.len() < 12 {
        return Err(EncryptionError::InvalidFormat);
    }

    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce_array: [u8; 12] = nonce_bytes
        .try_into()
        .map_err(|_| EncryptionError::InvalidFormat)?;
    let nonce = Nonce::from(nonce_array);
    let cipher = Aes256Gcm::new_from_slice(&key_bytes).unwrap();

    // Decrypt the data
    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

    String::from_utf8(plaintext)
        .map_err(|e| EncryptionError::DecryptionFailed(format!("Invalid UTF-8: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let key = generate_key();
        assert!(!key.is_empty());

        // Verify it's valid base64
        let decoded = BASE64.decode(&key).unwrap();
        assert_eq!(decoded.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = generate_key();
        let original_data = "my_secret_anilist_token_12345";

        let encrypted = encrypt(original_data, &key).unwrap();
        assert_ne!(encrypted, original_data);

        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(decrypted, original_data);
    }

    #[test]
    fn test_decrypt_with_wrong_key() {
        let key1 = generate_key();
        let key2 = generate_key();
        let data = "secret_data";

        let encrypted = encrypt(data, &key1).unwrap();
        let result = decrypt(&encrypted, &key2);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_key() {
        let result = encrypt("data", "invalid_base64!");
        assert!(result.is_err());
    }
}
