#[cfg(test)]
mod tests {
    use crate::auth::{jwt::JwtService, password::PasswordService, };
    use crate::core::models::User;

    #[test]
    fn test_argon2_password_hashing() {
        let hasher = PasswordService::new();
        let password = "test_password_123!";

        let hash = hasher.hash_password(password).unwrap();
        assert!(hasher.verify_password(&hash, password).unwrap());
        assert!(!hasher.verify_password(&hash, "wrong_password").unwrap());
    }

    #[test]
    fn test_password_hash_uniqueness() {
        let hasher = PasswordService::new();
        let password = "same_password";

        let hash1 = hasher.hash_password(password).unwrap();
        let hash2 = hasher.hash_password(password).unwrap();

        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(hasher.verify_password(&hash1, password).unwrap());
        assert!(hasher.verify_password(&hash2, password).unwrap());
    }

    #[test]
    fn test_jwt_token_creation_and_verification() {
        let jwt_service = JwtService::new("test-secret");

        let user = User::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            "password123",
            Some("admin".to_string()),
        ).unwrap();

        // Create token
        let token = jwt_service.create_token(&user).unwrap();
        assert!(!token.is_empty());

        // Verify token
        let claims = jwt_service.verify_token(&token).unwrap();
        assert_eq!(claims.sub, user.id);
        assert_eq!(claims.role, "admin");
    }

    #[test]
    fn test_user_model_with_argon2() {
        let user = User::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            "secure_password_123!",
            Some("user".to_string()),
        ).unwrap();

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.role, "user");
        assert!(user.verify_password("secure_password_123!").unwrap());
        assert!(!user.verify_password("wrong_password").unwrap());

        // Argon2 hashes should start with $argon2
        assert!(user.password_hash.starts_with("$argon2"));
    }
}