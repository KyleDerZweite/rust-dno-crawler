#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    rsx! {
        div { class: "max-w-md w-full mx-auto space-y-8",
            div { class: "text-center",
                h2 { class: "mt-6 text-3xl font-extrabold text-gray-900",
                    "Create your account"
                }
            }
            form {
                class: "mt-8 space-y-6 bg-white shadow-md rounded-lg p-6",
                method: "POST",
                action: "/register",
                div { class: "rounded-md shadow-sm space-y-4",
                    div {
                        label {
                            r#for: "email",
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Email address"
                        }
                        input {
                            id: "email",
                            name: "email",
                            r#type: "email",
                            required: true,
                            class: "appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm",
                            placeholder: "Email address"
                        }
                    }
                    div {
                        label {
                            r#for: "password",
                            class: "block text-sm font-medium text-gray-700 mb-1",
                            "Password"
                        }
                        input {
                            id: "password",
                            name: "password",
                            r#type: "password",
                            required: true,
                            class: "appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm",
                            placeholder: "Password"
                        }
                    }
                }
                div {
                    button {
                        r#type: "submit",
                        class: "group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                        "Sign up"
                    }
                }
                div { class: "text-center",
                    a {
                        href: "/login",
                        class: "font-medium text-indigo-600 hover:text-indigo-500",
                        "Already have an account? Sign in"
                    }
                }
            }
        }
    }
}