#[cfg(test)]
mod tests {
    use core::{database::Database, models::CreateUserRequest};
    use auth::jwt::JwtService;
    use crate::routes::create_api_routes;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use serde_json::json;

    #[tokio::test]
    async fn test_api_login_success() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        let jwt_service = JwtService::new("test-secret");

        // Create a test user
        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: Some("user".to_string()),
        };
        let _ = db.create_user(request).await.unwrap();

        let app = create_api_routes(db, jwt_service);

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "email": "test@example.com",
                    "password": "password123"
                }).to_string()
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_login_guest_forbidden() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        let jwt_service = JwtService::new("test-secret");

        // Create a guest user
        let request = CreateUserRequest {
            name: "Guest User".to_string(),
            email: "guest@example.com".to_string(),
            password: "password123".to_string(),
            role: Some("guest".to_string()),
        };
        let _ = db.create_user(request).await.unwrap();

        let app = create_api_routes(db, jwt_service);

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "email": "guest@example.com",
                    "password": "password123"
                }).to_string()
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}