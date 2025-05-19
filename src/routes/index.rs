#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::routes::Route; 

#[component]
pub fn Index() -> Element {
    rsx! {
        div {
            class: "p-4",
            h1 { class: "text-2xl font-bold text-blue-600", "DNO Crawler Dashboard" }
            p { "Welcome to the DNO Crawler. Database display, crawl management, and filtering will be implemented here." }
            nav {
                class: "mt-4 space-x-2",
                Link { to: Route::Login {}, "Login" }
                Link { to: Route::Admin {}, "Admin"}
                Link { to: Route::Register {}, "Register" }
            }
            // TODO: Implement database display, filtering, crawl starting UI
        }
    }
}