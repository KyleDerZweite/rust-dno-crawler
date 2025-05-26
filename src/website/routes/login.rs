#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    rsx! {
        head {
            title { "DNO Crawler - Login" }
            link { rel: "stylesheet", href: "/public/tailwind_output.css" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        }
        body { class: "bg-gray-100 min-h-screen flex flex-col",
            // Header
            header { class: "bg-white shadow-lg",
                nav { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "flex justify-between h-16",
                        div { class: "flex items-center",
                            a { 
                                href: "/",
                                class: "flex-shrink-0 flex items-center",
                                h1 { class: "text-xl font-bold text-indigo-600",
                                    "DNO - Crawler"
                                }
                            }
                        }
                    }
                }
            }
            
            // Main Content
            main { class: "flex-grow container mx-auto px-4 py-8",
                div { class: "max-w-md w-full mx-auto space-y-8",
                    div { class: "text-center",
                        h2 { class: "mt-6 text-3xl font-extrabold text-gray-900",
                            "Sign in to your account"
                        }
                    }
                    form { 
                        class: "mt-8 space-y-6 bg-white shadow-md rounded-lg p-6",
                        method: "POST",
                        action: "/login",
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
                                "Sign in"
                            }
                        }
                        div { class: "text-center",
                            a {
                                href: "/register",
                                class: "font-medium text-indigo-600 hover:text-indigo-500",
                                "Don't have an account? Sign up"
                            }
                        }
                    }
                }
            }
            
            // Footer
            footer { class: "bg-gray-800 text-white",
                div { class: "max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8 text-center",
                    p { class: "text-gray-400 text-sm",
                        "© 2025 Kyle. All rights reserved."
                    }
                }
            }
        }
    }
}