#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        div { "Hello from Dno Crawler!" }
    )
}