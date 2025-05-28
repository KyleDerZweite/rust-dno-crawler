#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
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
                        "Welcome back"
                    }
                    p {
                        class: "text-neutral-400 text-sm",
                        "Sign in to your KyleHub account"
                    }
                }

                // Login Form
                div {
                    class: "card mt-8",
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
                                    class: "h-4 w-4 text-green-500 focus:ring-green-500 border-neutral-600 rounded bg-neutral-700"
                                }
                                label {
                                    r#for: "remember-me",
                                    class: "ml-2 block text-sm text-neutral-400",
                                    "Remember me"
                                }
                            }
                            div {
                                a {
                                    href: "/forgot-password",
                                    class: "text-sm text-green-500 hover:text-green-400 transition-colors",
                                    "Forgot your password?"
                                }
                            }
                        }

                        // Submit Button
                        div {
                            button {
                                r#type: "submit",
                                class: "btn-primary w-full py-3 text-base font-medium",
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
                                div { class: "w-full border-t border-neutral-700" }
                            }
                            div {
                                class: "relative flex justify-center text-sm",
                                span {
                                    class: "px-2 bg-neutral-800 text-neutral-400",
                                    "New to KyleHub?"
                                }
                            }
                        }
                    }

                    // Register Link
                    div {
                        class: "mt-6 text-center",
                        a {
                            href: "/register",
                            class: "btn-secondary w-full py-3 text-base font-medium inline-block",
                            "Create an account"
                        }
                    }
                }
            }
        }
    }
}