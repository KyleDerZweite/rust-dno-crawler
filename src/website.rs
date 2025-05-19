#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Route;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}