#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-gray-100",
            div {
                class: "bg-white p-8 rounded shadow-md w-full max-w-md",
                h2 { class: "text-2xl font-bold mb-6 text-center", "Login" }
                // The actual login form would be an Axum handler, Dioxus just shows the page.
                // Or, you could have a Dioxus form that calls an API endpoint.
                // For now, linking to the Axum-handled form:
                form {
                    action: "/auth/login", // Axum route
                    method: "post",
                    class: "space-y-4",
                    div {
                        label { class: "block text-sm font-medium text-gray-700", r#for: "username", "Username" }
                        input { r#type: "text", name: "username", id: "username", class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" }
                    }
                    div {
                        label { class: "block text-sm font-medium text-gray-700", r#for: "password", "Password" }
                        input { r#type: "password", name: "password", id: "password", class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" }
                    }
                    div {
                        button {
                            r#type: "submit",
                            class: "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                            "Sign in"
                        }
                    }
                }
                p { class: "mt-4 text-center text-sm",
                    "Don't have an account? "
                    // Link { to: Route::Register {}, class: "font-medium text-indigo-600 hover:text-indigo-500", "Sign up" }
                    // For now, direct link to Axum handler if register page is also Axum-first
                     a { href: "/auth/register", class: "font-medium text-indigo-600 hover:text-indigo-500", "Sign up" }
                }
                Link { class: "block mt-2 text-center text-sm text-blue-500 hover:underline", to: Route::Index {}, "Back to Home"}
            }
        }
    }
}