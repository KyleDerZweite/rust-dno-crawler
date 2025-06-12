#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::ui::*;

#[component]
pub fn Register() -> Element {
    // Theme-aware classes using custom color palette
    let bg_color = "bg-dark-charcoal-800";
    let heading_color = "text-light-beige-200";
    let text_color = "text-light-beige-500";
    let label_color = "text-light-beige-300";
    let divider_bg = "bg-dark-charcoal-600";
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
                        "Join KyleHub"
                    }
                    p {
                        class: format!("{} text-sm", text_color),
                        "Create your account to get started"
                    }
                }

                // Register Form
                Card {
                    variant: CardVariant::Glass,
                    padding: CardPadding::Large,
                    class: "mt-8".to_string(),
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
                                Input {
                                    id: "name".to_string(),
                                    name: "name".to_string(),
                                    input_type: "text".to_string(),
                                    placeholder: "Enter your full name".to_string(),
                                    required: true,
                                }
                            }

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
                                    placeholder: "Create a secure password".to_string(),
                                    required: true,
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
                                Input {
                                    id: "confirm-password".to_string(),
                                    name: "confirm-password".to_string(),
                                    input_type: "password".to_string(),
                                    placeholder: "Confirm your password".to_string(),
                                    required: true,
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
                                    class: format!("h-4 w-4 text-forest-green-500 focus:ring-forest-green-500 rounded border-dark-charcoal-400 bg-dark-charcoal-500")
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
                                        class: "text-forest-green-400 hover:text-forest-green-300 transition-colors underline-offset-4 hover:underline",
                                        "Terms of Service"
                                    }
                                    " and "
                                    a {
                                        href: "/privacy",
                                        class: "text-forest-green-400 hover:text-forest-green-300 transition-colors underline-offset-4 hover:underline",
                                        "Privacy Policy"
                                    }
                                }
                            }
                        }

                        // Submit Button (Primary - Forest Green for new user registration)
                        div {
                            PrimaryButton {
                                size: ButtonSize::Large,
                                class: "w-full".to_string(),
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
                            class: "text-forest-green-400 hover:text-forest-green-300 font-medium transition-colors underline-offset-4 hover:underline",
                            "Sign in to your account"
                        }
                    }
                }
            }
        }
    }
}
