pub mod routes;
pub mod middleware;

// Re-export commonly used types
pub use routes::api_routes;
pub use middleware::{AuthenticatedUser, UserRole};
    
// Placeholder for AppState - this should be defined properly
#[derive(Clone, Default)]
pub struct AppState {
    // Add your application state fields here
    // For example:
    // pub database: DatabasePool,
    // pub config: AppConfig,
}