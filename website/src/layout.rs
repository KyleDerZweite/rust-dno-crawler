#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::{header::Header, footer::Footer};
use crate::routes::Route;

#[component]
pub fn Layout() -> Element {
    let user_role: Option<String> = None;
    let is_authenticated = false;
    
    rsx! {
        div { class: "bg-black min-h-screen flex flex-col",
            Header {
                user_role: user_role.clone(),
                is_authenticated: is_authenticated,
            }
            main { class: "flex-grow w-full pt-24 px-4",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
