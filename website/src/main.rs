use std::{env, net::SocketAddr};
use axum::{routing::get, Router, response::Html};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::{info, Level};
use tracing_subscriber;
use dioxus::prelude::*;

// Import website components directly
use website::{
    layout::{Layout, LayoutProps},
    routes::{
        home::Home,
        dashboard::Dashboard,
        error_404::Error404,
        login::Login,
        register::Register,
        privacy::Privacy,
        terms::Terms,
        contact::Contact,
        impressum::Impressum,
        demo::Demo,
    },
};

// Helper function to render with layout (standalone mode - no auth)
fn render_with_layout(content: Element) -> String {
    let mut app = VirtualDom::new_with_props(Layout, LayoutProps {
        user_role: None,
        is_authenticated: false,
        children: content,
    });

    let _ = app.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&mut app);
    format!("<!DOCTYPE html>{}", html)
}

// Standalone handlers (no database/auth required)
async fn home_page() -> Html<String> {
    let content = rsx! { Home {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn dashboard_page() -> Html<String> {
    let content = rsx! { 
        Dashboard {
            name: "Demo User".to_string(),
            email: "demo@example.com".to_string(),
            role: "user".to_string(),
        }
    };
    let html = render_with_layout(content);
    Html(html)
}

async fn login_page() -> Html<String> {
    let content = rsx! { Login {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn register_page() -> Html<String> {
    let content = rsx! { Register {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn privacy_page() -> Html<String> {
    let content = rsx! { Privacy {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn terms_page() -> Html<String> {
    let content = rsx! { Terms {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn contact_page() -> Html<String> {
    let content = rsx! { Contact {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn impressum_page() -> Html<String> {
    let content = rsx! { Impressum {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn demo_page() -> Html<String> {
    let content = rsx! { Demo {} };
    let html = render_with_layout(content);
    Html(html)
}

async fn error_404_page() -> Html<String> {
    let content = rsx! { Error404 {} };
    let html = render_with_layout(content);
    Html(html)
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenv::dotenv(); // Ignore errors if .env file doesn't exist

        Config {
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load configuration
    let config = Config::from_env();
    info!("Starting Website server in standalone mode (no auth/database)");

    // Website routes (no auth required in standalone mode)
    let website_routes = Router::new()
        .route("/", get(home_page))
        .route("/dashboard", get(dashboard_page))
        .route("/login", get(login_page))
        .route("/register", get(register_page))
        .route("/privacy", get(privacy_page))
        .route("/terms", get(terms_page))
        .route("/contact", get(contact_page))
        .route("/impressum", get(impressum_page))
        .route("/demo", get(demo_page))
        .fallback(error_404_page);

    // Static file serving - adjust path based on working directory
    let public_path = if std::path::Path::new("./website/public").exists() {
        "./website/public"  // Running from project root
    } else {
        "./public"  // Running from website directory
    };
    let static_routes = Router::new()
        .nest_service("/public", ServeDir::new(public_path));

    // Combine all routes
    let app = Router::new()
        .merge(website_routes)
        .merge(static_routes)
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::new(
        config.server_host.parse()?,
        config.server_port,
    );

    info!("Website server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}