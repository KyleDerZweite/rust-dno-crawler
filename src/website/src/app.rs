use dioxus::prelude::*;
use crate::router::Route;

#[component]
pub fn App() -> Element {
    rsx!(
        document::Stylesheet { href: asset!("./public/tailwind_output.css") }
        Router::<Route> {}
    )
}