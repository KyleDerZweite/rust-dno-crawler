#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Register() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-neutral-900 py-12 px-4 sm:px-6 lg:px-8",
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
                        class: "text-3xl font-bold text-neutral-100 mb-2",
                        "Join KyleHub"
                    }
                    p {
                        class: "text-neutral-400 text-sm",
                        "Create your account to get started"
                    }
                }

                // Register Form
                div {
                    class: "card mt-8",
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
                                    class: "block text-sm font-medium text-neutral-100 mb-2",
                                    "Full name"
                                }
                                input {
                                    id: "name",
                                    name: "name",
                                    r#type: "text",
                                    required: true,
                                    class: "input-field w-full",
                                    placeholder: "Enter your full name"
                                }
                            }

                            // Email Field
                            div {
                                label {
                                    r#for: "email",
                                    class: "block text-sm font-medium text-neutral-100 mb-2",
                                    "Email address"
                                }
                                input {
                                    id: "email",
                                    name: "email",
                                    r#type: "email",
                                    required: true,
                                    class: "input-field w-full",
                                    placeholder: "Enter your email address"
                                }
                            }

                            // Password Field
                            div {
                                label {
                                    r#for: "password",
                                    class: "block text-sm font-medium text-neutral-100 mb-2",
                                    "Password"
                                }
                                input {
                                    id: "password",
                                    name: "password",
                                    r#type: "password",
                                    required: true,
                                    class: "input-field w-full",
                                    placeholder: "Create a secure password"
                                }
                                p {
                                    class: "mt-1 text-xs text-neutral-400",
                                    "At least 8 characters with numbers and symbols"
                                }
                            }

                            // Confirm Password Field
                            div {
                                label {
                                    r#for: "confirm-password",
                                    class: "block text-sm font-medium text-neutral-100 mb-2",
                                    "Confirm password"
                                }
                                input {
                                    id: "confirm-password",
                                    name: "confirm-password",
                                    r#type: "password",
                                    required: true,
                                    class: "input-field w-full",
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
                                    class: "h-4 w-4 text-green-500 focus:ring-green-500 border-neutral-600 rounded bg-neutral-700"
                                }
                            }
                            div {
                                class: "ml-3 text-sm",
                                label {
                                    r#for: "terms",
                                    class: "text-neutral-400",
                                    "I agree to the "
                                    a {
                                        href: "/terms",
                                        class: "text-green-500 hover:text-green-400 transition-colors",
                                        "Terms of Service"
                                    }
                                    " and "
                                    a {
                                        href: "/privacy",
                                        class: "text-green-500 hover:text-green-400 transition-colors",
                                        "Privacy Policy"
                                    }
                                }
                            }
                        }

                        // Submit Button
                        div {
                            button {
                                r#type: "submit",
                                class: "btn-primary w-full py-3 text-base font-medium",
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
                                div { class: "w-full border-t border-neutral-700" }
                            }
                            div {
                                class: "relative flex justify-center text-sm",
                                span {
                                    class: "px-2 bg-neutral-800 text-neutral-400",
                                    "Already have an account?"
                                }
                            }
                        }
                    }

                    // Login Link
                    div {
                        class: "mt-6 text-center",
                        a {
                            href: "/login",
                            class: "text-green-500 hover:text-green-400 font-medium transition-colors",
                            "Sign in to your account"
                        }
                    }
                }
            }
        }
    }
}