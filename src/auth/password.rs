use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::core::errors::AppError;

#[derive(Clone, Debug)]
pub struct PasswordService {
    argon2: Argon2<'static>,
}

impl PasswordService {
    pub fn new() -> Self {
        Self {
            // Argon2 with default params (Argon2id v19)
            // This provides good security with reasonable performance
            argon2: Argon2::default(),
        }
    }

    /// Hash a password using Argon2id with a random salt
    pub fn hash_password(&self, password: &str) -> Result<String, AppError> {
        // Generate a random salt
        let salt = SaltString::generate(&mut OsRng);

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))?
            .to_string();

        Ok(password_hash)
    }

    /// Verify a password against a stored hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        // Parse the stored hash
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalServerError(format!("Invalid password hash format: {}", e)))?;

        // Verify password against PHC string
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false), // Password doesn't match
        }
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let service = PasswordService::new();
        let password = "super_secure_password_123!";

        // Hash the password
        let hash = service.hash_password(password).unwrap();

        // Verify correct password
        assert!(service.verify_password(password, &hash).unwrap());

        // Verify incorrect password
        assert!(!service.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_different_passwords_generate_different_hashes() {
        let service = PasswordService::new();
        let password = "test_password";

        let hash1 = service.hash_password(password).unwrap();
        let hash2 = service.hash_password(password).unwrap();

        // Salts should be different, so hashes should be different
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(service.verify_password(password, &hash1).unwrap());
        assert!(service.verify_password(password, &hash2).unwrap());
    }

    #[test]
    fn test_hash_format() {
        let service = PasswordService::new();
        let password = "test_password";

        let hash = service.hash_password(password).unwrap();

        // Argon2id hashes should start with $argon2id$
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn test_empty_password() {
        let service = PasswordService::new();
        let password = "";

        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("not_empty", &hash).unwrap());
    }

    #[test]
    fn test_invalid_hash_format() {
        let service = PasswordService::new();
        let result = service.verify_password("password", "invalid_hash");

        assert!(result.is_err());
    }

    #[test]
    fn test_unicode_password() {
        let service = PasswordService::new();
        let password = "пароль123🔒";

        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("password123", &hash).unwrap());
    }

    #[test]
    fn test_long_password() {
        let service = PasswordService::new();
        let password = "a".repeat(1000); // Very long password

        let hash = service.hash_password(&password).unwrap();
        assert!(service.verify_password(&password, &hash).unwrap());
    }
}