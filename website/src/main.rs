#![allow(non_snake_case)]
//! Development entry point for hot-reload development with `dioxus serve`
//! 
//! Use `dx serve` to start the development server with hot-reload support.
//! For production, the full application stack in the `app` crate should be used.

use dioxus::prelude::*;

mod components;
mod routes;
mod layout;

use layout::Layout;
use routes::Route;

/// Main development app entry point for `dioxus serve`
fn main() {
    launch(App);
}

/// Main app component with router and layout
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}