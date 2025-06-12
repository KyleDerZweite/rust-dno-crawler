#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::ui::*;

#[component]
pub fn Login() -> Element {
    // Theme-aware classes using custom color palette
    let bg_color = "bg-dark-charcoal-800";
    let heading_color = "text-light-beige-200";
    let text_color = "text-light-beige-500";
    let label_color = "text-light-beige-300";
    let divider_bg = "bg-dark-charcoal-500";
    let divider_border = "border-dark-charcoal-400";

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
                            class: "w-12 h-12 bg-forest-green-500 rounded-xl flex items-center justify-center",
                            span { class: "text-light-beige-200 font-bold text-xl", "K" }
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
                Card {
                    variant: CardVariant::Glass,
                    padding: CardPadding::Large,
                    class: "mt-8".to_string(),
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
                                Input {
                                    id: "email".to_string(),
                                    name: "email".to_string(),
                                    input_type: "email".to_string(),
                                    placeholder: "Enter your email address".to_string(),
                                    required: true,
                                }
                            }

                            // Password Field
                            div {
                                label {
                                    r#for: "password",
                                    class: format!("block text-sm font-medium {} mb-2", label_color),
                                    "Password"
                                }
                                Input {
                                    id: "password".to_string(),
                                    name: "password".to_string(),
                                    input_type: "password".to_string(),
                                    placeholder: "Enter your password".to_string(),
                                    required: true,
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
                                    class: format!("h-4 w-4 text-forest-green-500 focus:ring-forest-green-500 rounded border-dark-charcoal-400 bg-dark-charcoal-500")
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
                                    class: "text-sm text-forest-green-400 hover:text-forest-green-300 transition-colors underline-offset-4 hover:underline",
                                    "Forgot your password?"
                                }
                            }
                        }

                        // Submit Button (Primary - Green)
                        div {
                            PrimaryButton {
                                size: ButtonSize::Large,
                                class: "w-full".to_string(),
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
                        SecondaryButton {
                            href: "/register".to_string(),
                            size: ButtonSize::Large,
                            class: "w-full".to_string(),
                            "Create an account"
                        }
                    }
                }
            }
        }
    }
}
