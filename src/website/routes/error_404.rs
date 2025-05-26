#![allow(non_snake_case)]
use dioxus::prelude::*;

use header::Header;
use crate::website::components::header;

#[derive(Props, Clone, PartialEq)]
pub struct Error404Props {
    #[props(default = Vec::new())]
    pub route: Vec<String>,
}

#[component]
pub fn Error404(props: Error404Props) -> Element {
    rsx! {
        Header {}
        div { class: "flex-grow flex items-center justify-center",
            div { class: "text-center bg-white rounded-lg shadow-lg p-8 max-w-md mx-auto",
                div { class: "text-6xl font-bold text-red-500 mb-4", "404" }
                h1 { class: "text-2xl font-bold text-gray-900 mb-4", "Page not found" }
                p { class: "text-gray-600 mb-6", 
                    "We are terribly sorry, but the page you requested doesn't exist." 
                }
                pre { class: "text-xs text-red-600 bg-red-50 p-2 rounded mb-4",
                    "log:\nattempted to navigate to: {props.route:?}"
                }
                a {
                    href: "/login",
                    class: "inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                    "Go to Login"
                }
            }
        }
    }
}