// Standalone mode - minimal handlers without authentication
use crate::{
    layout::{Layout, LayoutProps},
    routes::{
        dashboard::Dashboard,
        error_404::Error404,
        login::Login,
        register::Register,
        privacy::Privacy,
        terms::Terms,
        contact::Contact,
        impressum::Impressum,
        home::Home,
    },
};
use axum::response::Html;
use dioxus::prelude::*;

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
pub async fn home_page() -> Html<String> {
    let content = rsx! { Home {} };
    let html = render_with_layout(content);
    Html(html)
}

pub async fn dashboard_page() -> Html<String> {
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

pub async fn login_page() -> Html<String> {
    let content = rsx! { Login {} };
    let html = render_with_layout(content);
    Html(html)
}

pub async fn register_page() -> Html<String> {
    let content = rsx! { Register {} };
    let html = render_with_layout(content);
    Html(html)
}

pub async fn privacy_page() -> Html<String> {
    let content = rsx! { Privacy {} };
    let html = render_with_layout(content);
    Html(html)
}

pub async fn terms_page() -> Html<String> {
    let content = rsx! { Terms {} };
    let html = render_with_layout(content);
    Html(html)
}

pub async fn contact_page() -> Html<String> {
    let content = rsx! { Contact {} };
    let html = render_with_layout(content);
    Html(html)
}

pub async fn impressum_page() -> Html<String> {
    let content = rsx! { Impressum {} };
    let html = render_with_layout(content);
    Html(html)
}

pub async fn error_404_page() -> Html<String> {
    let content = rsx! { Error404 { route: vec!["unknown".to_string()] } };
    let html = render_with_layout(content);
    Html(html)
}