#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::website::{
    components::{header::Header, footer::Footer},
    routes::Route,
};

#[derive(Props, Clone, PartialEq)]
pub struct AppProps {
    #[props(default = None)]
    pub user_email: Option<String>,
    #[props(default = None)]
    pub user_role: Option<String>,
    #[props(default = false)]
    pub is_authenticated: bool,
}

#[component]
pub fn App(props: AppProps) -> Element {
    rsx! {
        head {
            title { "DNO Crawler" }
            link { rel: "stylesheet", href: "/public/tailwind_output.css" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        }
        body { class: "bg-gray-100 min-h-screen flex flex-col",
            Router::<Route> {}
        }
    }
}

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col",
            Header {}
            main { class: "flex-grow container mx-auto px-4 py-8",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}