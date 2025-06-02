#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    // Theme-aware classes (simplified to dark theme)
    let bg_color = "bg-neutral-900";
    let card_bg = "bg-neutral-800 shadow-xl border border-neutral-700";
    let heading_color = "text-neutral-100";
    let text_color = "text-neutral-400";
    let label_color = "text-neutral-100";
    let input_bg = "bg-neutral-700 border-neutral-600 text-neutral-100 focus:border-green-500 focus:ring-green-500";
    let divider_bg = "bg-neutral-800";
    let divider_border = "border-neutral-700";

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
                        "Join KyleHub"
                    }
                    p {
                        class: format!("{} text-sm", text_color),
                        "Create your account to get started"
                    }
                }

                // Register Form
                div {
                    class: format!("{} rounded-2xl p-8 mt-8", card_bg),
                    form {
                        class: "space-y-6",
                        method: "POST",
                        action: "/register",

                        div {
                            class: "space-y-4",

                            // Full Name Field
                            div {
                                label {
                                    r#for: "name",
                                    class: format!("block text-sm font-medium {} mb-2", label_color),
                                    "Full name"
                                }
                                input {
                                    id: "name",
                                    name: "name",
                                    r#type: "text",
                                    required: true,
                                    class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", input_bg),
                                    placeholder: "Enter your full name"
                                }
                            }

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
                                    placeholder: "Create a secure password"
                                }
                                p {
                                    class: format!("mt-1 text-xs {}", text_color),
                                    "At least 8 characters with numbers and symbols"
                                }
                            }

                            // Confirm Password Field
                            div {
                                label {
                                    r#for: "confirm-password",
                                    class: format!("block text-sm font-medium {} mb-2", label_color),
                                    "Confirm password"
                                }
                                input {
                                    id: "confirm-password",
                                    name: "confirm-password",
                                    r#type: "password",
                                    required: true,
                                    class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", input_bg),
                                    placeholder: "Confirm your password"
                                }
                            }
                        }

                        // Terms and Privacy
                        div {
                            class: "flex items-start",
                            div {
                                class: "flex items-center h-5",
                                input {
                                    id: "terms",
                                    name: "terms",
                                    r#type: "checkbox",
                                    required: true,
                                    class: format!("h-4 w-4 text-green-500 focus:ring-green-500 rounded border-neutral-600 bg-neutral-700")
                                }
                            }
                            div {
                                class: "ml-3 text-sm",
                                label {
                                    r#for: "terms",
                                    class: format!("{}", text_color),
                                    "I agree to the "
                                    a {
                                        href: "/terms",
                                        class: "text-green-500 hover:text-green-600 transition-colors underline-offset-4 hover:underline",
                                        "Terms of Service"
                                    }
                                    " and "
                                    a {
                                        href: "/privacy",
                                        class: "text-green-500 hover:text-green-600 transition-colors underline-offset-4 hover:underline",
                                        "Privacy Policy"
                                    }
                                }
                            }
                        }

                        // Submit Button (Primary - Green for new user registration)
                        div {
                            button {
                                r#type: "submit",
                                class: "w-full bg-green-500 hover:bg-green-600 text-white py-3 px-4 rounded-xl text-base font-medium transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105",
                                "Create account"
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
                                    "Already have an account?"
                                }
                            }
                        }
                    }

                    // Login Link (Green hover for existing user action)
                    div {
                        class: "mt-6 text-center",
                        a {
                            href: "/login",
                            class: "text-green-500 hover:text-green-600 font-medium transition-colors underline-offset-4 hover:underline",
                            "Sign in to your account"
                        }
                    }
                }
            }
        }
    }
}
