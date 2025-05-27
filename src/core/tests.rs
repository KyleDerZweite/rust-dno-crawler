#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{database::Database, models::*, errors::AppError};
    use tokio;

    #[tokio::test]
    async fn test_database_connection() {
        let db = Database::new("sqlite::memory:").await;
        assert!(db.is_ok());
    }

    #[tokio::test]
    async fn test_user_creation() {
        let db = Database::new("sqlite::memory:").await.unwrap();

        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: None,
        };

        let result = db.create_user(request).await;
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.role, "user");
        assert!(!user.id.is_empty());
    }

    #[tokio::test]
    async fn test_user_duplicate_email() {
        let db = Database::new("sqlite::memory:").await.unwrap();

        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: None,
        };

        // First creation should succeed
        let _ = db.create_user(request.clone()).await.unwrap();

        // Second creation should fail
        let result = db.create_user(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_user_password_verification() {
        let user = User::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            "password123",
            None,
        ).unwrap();

        assert!(user.verify_password("password123").unwrap());
        assert!(!user.verify_password("wrongpassword").unwrap());
    }

    #[tokio::test]
    async fn test_get_user_by_email() {
        let db = Database::new("sqlite::memory:").await.unwrap();

        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: Some("admin".to_string()),
        };

        let created_user = db.create_user(request).await.unwrap();

        let found_user = db.get_user_by_email("test@example.com").await.unwrap();
        assert!(found_user.is_some());

        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.email, "test@example.com");
        assert_eq!(found_user.role, "admin");
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let db = Database::new("sqlite::memory:").await.unwrap();

        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: None,
        };

        let created_user = db.create_user(request).await.unwrap();

        let found_user = db.get_user_by_id(&created_user.id).await.unwrap();
        assert!(found_user.is_some());

        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.email, "test@example.com");
    }
}