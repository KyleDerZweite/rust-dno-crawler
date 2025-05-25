#[cfg(test)]
mod tests {
    use crate::auth::{jwt::JwtService, session::SessionUser};
    use crate::core::models::User;

    #[test]
    fn test_jwt_token_creation_and_verification() {
        let jwt_service = JwtService::new("test-secret");

        let user = User::new(
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
    fn test_jwt_invalid_token() {
        let jwt_service = JwtService::new("test-secret");

        let result = jwt_service.verify_token("invalid-token");
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_extract_token_from_header() {
        let result = JwtService::extract_token_from_header("Bearer test-token");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-token");

        let result = JwtService::extract_token_from_header("Invalid header");
        assert!(result.is_err());
    }

    #[test]
    fn test_session_user_permissions() {
        let admin_user = SessionUser {
            id: "1".to_string(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
        };

        let regular_user = SessionUser {
            id: "2".to_string(),
            email: "user@example.com".to_string(),
            role: "user".to_string(),
        };

        // Test admin permissions
        assert!(admin_user.is_admin());
        assert!(admin_user.is_user());
        assert!(admin_user.can_access_api());
        assert!(admin_user.can_manage_users());

        // Test regular user permissions
        assert!(!regular_user.is_admin());
        assert!(regular_user.is_user());
        assert!(regular_user.can_access_api());
        assert!(!regular_user.can_manage_users());
    }
}