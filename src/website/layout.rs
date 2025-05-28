#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::website::components::{header::Header, footer::Footer};

#[derive(Props, Clone, PartialEq)]
pub struct LayoutProps {
    #[props(default = None)]
    pub user_role: Option<String>,
    #[props(default = false)]
    pub is_authenticated: bool,
    pub children: Element,
}

#[component]
pub fn Layout(props: LayoutProps) -> Element {
    rsx! {
        head {
            title { "DNO Crawler" }
            link { rel: "stylesheet", href: "/public/tailwind_output.css" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        }
        div { class: "bg-gray-100 min-h-screen flex flex-col",
            Header {
                user_role: props.user_role.clone(),
                is_authenticated: props.is_authenticated,
            }
            main { class: "flex-grow w-full",
                {props.children}
            }
            Footer {}
        }
    }
}
