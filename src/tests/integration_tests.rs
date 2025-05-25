#[cfg(test)]
mod tests {
    use crate::{
        core::database::Database,
        auth::jwt::JwtService,
        api::routes::create_api_routes,
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use serde_json::json;

    #[tokio::test]
    async fn test_api_register_endpoint() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        let jwt_service = JwtService::new("test-secret");
        let app = create_api_routes(db, jwt_service);

        let request = Request::builder()
            .method("POST")
            .uri("/register")
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
    async fn test_api_login_endpoint() {
        let db = Database::new("sqlite::memory:").await.unwrap();
        let jwt_service = JwtService::new("test-secret");

        // First register a user
        let request = crate::core::models::CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: None,
        };
        let _ = db.create_user(request).await.unwrap();

        let app = create_api_routes(db, jwt_service);

        let request = Request::builder()
            .method("POST")
            .uri("/login")
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
}