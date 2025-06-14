#![allow(non_snake_case)]
use dioxus::prelude::*;
// Simplified without custom components

#[component]
pub fn Register() -> Element {
    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center py-12 px-4",
            div {
                class: "max-w-md w-full bg-gray-900/80 backdrop-blur-lg border border-white/10 p-8 rounded-xl shadow-2xl",

                // Header
                div {
                    class: "text-center",
                    div {
                        class: "flex justify-center mb-6",
                        div {
                            class: "w-12 h-12 bg-gradient-to-br from-emerald-500 to-emerald-600 rounded-xl flex items-center justify-center shadow-lg",
                            span { class: "text-white font-bold text-xl", "K" }
                        }
                    }
                    h2 {
                        class: "text-3xl font-bold text-white mb-2",
                        "Join KyleHub"
                    }
                    p {
                        class: "text-gray-400 text-sm",
                        "Create your account to get started"
                    }
                }

                // Register Form
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
                                class: "block text-sm font-medium text-gray-300 mb-2",
                                "Full name"
                            }
                            input {
                                id: "name",
                                name: "name",
                                r#type: "text",
                                required: true,
                                class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white placeholder-gray-400 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-all duration-300",
                                placeholder: "Enter your full name"
                            }
                        }

                        // Email Field
                        div {
                            label {
                                r#for: "email",
                                class: "block text-sm font-medium text-gray-300 mb-2",
                                "Email address"
                            }
                            input {
                                id: "email",
                                name: "email",
                                r#type: "email",
                                required: true,
                                class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white placeholder-gray-400 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-all duration-300",
                                placeholder: "Enter your email address"
                            }
                        }

                        // Password Field
                        div {
                            label {
                                r#for: "password",
                                class: "block text-sm font-medium text-gray-300 mb-2",
                                "Password"
                            }
                            input {
                                id: "password",
                                name: "password",
                                r#type: "password",
                                required: true,
                                class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white placeholder-gray-400 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-all duration-300",
                                placeholder: "Create a secure password"
                            }
                            p {
                                class: "mt-1 text-xs text-gray-400",
                                "At least 8 characters with numbers and symbols"
                            }
                        }

                        // Confirm Password Field
                        div {
                            label {
                                r#for: "confirm-password",
                                class: "block text-sm font-medium text-gray-300 mb-2",
                                "Confirm password"
                            }
                            input {
                                id: "confirm-password",
                                name: "confirm-password",
                                r#type: "password",
                                required: true,
                                class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white placeholder-gray-400 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-all duration-300",
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
                                class: "h-4 w-4 text-emerald-500 focus:ring-emerald-500 border-gray-400 rounded bg-gray-700"
                            }
                        }
                        div {
                            class: "ml-3 text-sm",
                            label {
                                r#for: "terms",
                                class: "text-gray-400",
                                "I agree to the "
                                a {
                                    href: "/terms",
                                    class: "text-emerald-400 hover:text-emerald-300 transition-colors hover:underline",
                                    "Terms of Service"
                                }
                                " and "
                                a {
                                    href: "/privacy",
                                    class: "text-emerald-400 hover:text-emerald-300 transition-colors hover:underline",
                                    "Privacy Policy"
                                }
                            }
                        }
                    }

                    // Submit Button
                    button {
                        r#type: "submit",
                        class: "w-full bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-semibold py-3 px-6 rounded-xl transition-all duration-300 focus:ring-2 focus:ring-emerald-500 focus:outline-none shadow-lg hover:shadow-xl transform hover:scale-[1.02]",
                        "Create account"
                    }
                }

                // Login Link
                div {
                    class: "text-center pt-4 border-t border-gray-700",
                    p {
                        class: "text-gray-400 text-sm",
                        "Already have an account? "
                        a {
                            href: "/login",
                            class: "text-emerald-400 hover:text-emerald-300 transition-colors hover:underline font-medium",
                            "Sign in"
                        }
                    }
                }
            }
        }
    }
}
