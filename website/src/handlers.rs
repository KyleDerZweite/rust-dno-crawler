// Production mode handlers for SSR
use crate::routes::{
    dashboard::Dashboard,
    error_404::Error404,
    login::Login,
    register::Register,
    privacy::Privacy,
    terms::Terms,
    contact::Contact,
    impressum::Impressum,
    home::Home,
};
use axum::response::Html;
use dioxus::prelude::*;

// Create an app component that includes layout
#[component]
fn AppWithLayout(children: Element) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("public/tailwind_output.css") }
        div { class: "bg-dark-charcoal-700 min-h-screen flex flex-col",
            crate::components::header::Header {
                user_role: None,
                is_authenticated: false,
            }
            main { class: "flex-grow w-full pt-20",
                {children}
            }
            crate::components::footer::Footer {}
        }
    }
}

// Helper function to render a component with layout
fn render_component_with_layout(content: Element) -> String {
    let props = AppWithLayoutProps { children: content };
    let mut app = VirtualDom::new_with_props(AppWithLayout, props);
    let _ = app.rebuild(&mut dioxus_core::NoOpMutations);
    let html = dioxus_ssr::render(&mut app);
    format!("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>DNO Crawler</title></head><body>{}</body></html>", html)
}

// Route handlers for production SSR
pub async fn home_page() -> Html<String> {
    let content = rsx! { Home {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn dashboard_page() -> Html<String> {
    let content = rsx! { Dashboard {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn login_page() -> Html<String> {
    let content = rsx! { Login {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn register_page() -> Html<String> {
    let content = rsx! { Register {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn privacy_page() -> Html<String> {
    let content = rsx! { Privacy {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn terms_page() -> Html<String> {
    let content = rsx! { Terms {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn contact_page() -> Html<String> {
    let content = rsx! { Contact {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn impressum_page() -> Html<String> {
    let content = rsx! { Impressum {} };
    let html = render_component_with_layout(content);
    Html(html)
}

pub async fn not_found_page() -> Html<String> {
    let content = rsx! { Error404 { route: vec![] } };
    let html = render_component_with_layout(content);
    Html(html)
}