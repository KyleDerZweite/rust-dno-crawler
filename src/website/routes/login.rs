#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::website::theme::{use_theme, Theme};

#[component]
pub fn Login() -> Element {
    let theme = use_theme();

    // Theme-aware classes
    let bg_color = match theme() {
        Theme::Light => "bg-gray-100",
        Theme::Dark => "bg-neutral-900",
    };

    let card_bg = match theme() {
        Theme::Light => "bg-white shadow-lg border border-gray-200",
        Theme::Dark => "bg-neutral-800 shadow-xl border border-neutral-700",
    };

    let heading_color = match theme() {
        Theme::Light => "text-neutral-800",
        Theme::Dark => "text-neutral-100",
    };

    let text_color = match theme() {
        Theme::Light => "text-neutral-600",
        Theme::Dark => "text-neutral-400",
    };

    let label_color = match theme() {
        Theme::Light => "text-neutral-700",
        Theme::Dark => "text-neutral-100",
    };

    let input_bg = match theme() {
        Theme::Light => "bg-white border-gray-300 text-neutral-800 focus:border-green-500 focus:ring-green-500",
        Theme::Dark => "bg-neutral-700 border-neutral-600 text-neutral-100 focus:border-green-500 focus:ring-green-500",
    };

    let divider_bg = match theme() {
        Theme::Light => "bg-white",
        Theme::Dark => "bg-neutral-800",
    };

    let divider_border = match theme() {
        Theme::Light => "border-gray-200",
        Theme::Dark => "border-neutral-700",
    };

    // Secondary button with green hover (for register link)
    let secondary_button = match theme() {
        Theme::Light => "border-neutral-300 text-neutral-700 hover:text-green-500 hover:bg-green-50 hover:border-green-500 bg-transparent",
        Theme::Dark => "border-neutral-600 text-neutral-300 hover:text-green-500 hover:bg-green-900/20 hover:border-green-500 bg-transparent",
    };

    rsx! {
        div {
            class: format!("min-h-screen flex items-center justify-center {} py-12 px-4 sm:px-6 lg:px-8", bg_color),
            div {
                class: "max-w-md w-full space-y-8",

                // Header Section
                div {
                    class: "text-center",
                    // Logo
                    div {
                        class: "flex justify-center mb-6",
                        div {
                            class: "w-12 h-12 bg-green-500 rounded-xl flex items-center justify-center",
                            span { class: "text-white font-bold text-xl", "K" }
                        }
                    }
                    h2 {
                        class: format!("text-3xl font-bold {} mb-2", heading_color),
                        "Welcome back"
                    }
                    p {
                        class: format!("{} text-sm", text_color),
                        "Sign in to your KyleHub account"
                    }
                }

                // Login Form
                div {
                    class: format!("{} rounded-2xl p-8 mt-8", card_bg),
                    form {
                        class: "space-y-6",
                        method: "POST",
                        action: "/login",

                        div {
                            class: "space-y-4",

                            // Email Field
                            div {
                                label {
                                    r#for: "email",
                                    class: format!("block text-sm font-medium {} mb-2", label_color),
                                    "Email address"
                                }
                                input {
                                    id: "email",
                                    name: "email",
                                    r#type: "email",
                                    required: true,
                                    class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", input_bg),
                                    placeholder: "Enter your email address"
                                }
                            }

                            // Password Field
                            div {
                                label {
                                    r#for: "password",
                                    class: format!("block text-sm font-medium {} mb-2", label_color),
                                    "Password"
                                }
                                input {
                                    id: "password",
                                    name: "password",
                                    r#type: "password",
                                    required: true,
                                    class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", input_bg),
                                    placeholder: "Enter your password"
                                }
                            }
                        }

                        // Remember Me & Forgot Password
                        div {
                            class: "flex items-center justify-between",
                            div {
                                class: "flex items-center",
                                input {
                                    id: "remember-me",
                                    name: "remember-me",
                                    r#type: "checkbox",
                                    class: format!("h-4 w-4 text-green-500 focus:ring-green-500 rounded {}", match theme() {
                                        Theme::Light => "border-gray-300 bg-white",
                                        Theme::Dark => "border-neutral-600 bg-neutral-700",
                                    })
                                }
                                label {
                                    r#for: "remember-me",
                                    class: format!("ml-2 block text-sm {}", text_color),
                                    "Remember me"
                                }
                            }
                            div {
                                a {
                                    href: "/forgot-password",
                                    class: "text-sm text-green-500 hover:text-green-600 transition-colors underline-offset-4 hover:underline",
                                    "Forgot your password?"
                                }
                            }
                        }

                        // Submit Button (Primary - Green)
                        div {
                            button {
                                r#type: "submit",
                                class: "w-full bg-green-500 hover:bg-green-600 text-white py-3 px-4 rounded-xl text-base font-medium transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105",
                                "Sign in"
                            }
                        }
                    }

                    // Divider
                    div {
                        class: "mt-6",
                        div {
                            class: "relative",
                            div {
                                class: "absolute inset-0 flex items-center",
                                div { class: format!("w-full border-t {}", divider_border) }
                            }
                            div {
                                class: "relative flex justify-center text-sm",
                                span {
                                    class: format!("px-2 {} {}", divider_bg, text_color),
                                    "New to KyleHub?"
                                }
                            }
                        }
                    }

                    // Register Link (Secondary - Green hover for user registration)
                    div {
                        class: "mt-6 text-center",
                        a {
                            href: "/register",
                            class: format!("w-full py-3 px-4 rounded-xl text-base font-medium inline-block transition-all duration-200 border-2 {}", secondary_button),
                            "Create an account"
                        }
                    }
                }
            }
        }
    }
}