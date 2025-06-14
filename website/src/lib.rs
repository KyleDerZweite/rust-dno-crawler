pub mod components;
pub mod layout;
pub mod routes;

// Export main app component for integration
pub use routes::Route;

// Handlers module for SSR production mode
#[cfg(feature = "server")]
pub mod handlers;

// Export main app component for external use
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}