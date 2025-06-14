#![allow(non_snake_case)]
//! Entry point for both development (`dx serve`) and production (SSR with auth)

use dioxus::prelude::*;

mod components;
mod routes;
mod layout;

use routes::Route;

// Development mode: Simple client-side app
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Stylesheet { href: asset!("public/tailwind_output.css") }
            Router::<Route> {}
        }
    });
}

// Production mode: SSR server with auth
#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::Router as AxumRouter;
    use dioxus_ssr::*;
    use tower_http::services::ServeDir;
    use std::net::SocketAddr;
    
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Build the Axum app with static file serving and SSR
    let app = AxumRouter::new()
        .serve_dioxus_application(ServeConfig::builder().build(), || {
            rsx! {
                Router::<Route> {}
            }
        })
        .nest_service("/public", ServeDir::new("public"));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Website server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}