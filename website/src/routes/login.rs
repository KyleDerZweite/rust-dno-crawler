#![allow(non_snake_case)]
use dioxus::prelude::*;
// Simplified without custom components

#[component]
pub fn Login() -> Element {
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
                        "Welcome back"
                    }
                    p {
                        class: "text-gray-400 text-sm",
                        "Sign in to your account"
                    }
                }

                // Login Form
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
                                class: "block text-sm font-medium text-gray-300 mb-2",
                                "Email address"
                            }
                            input {
                                id: "email",
                                name: "email",
                                r#type: "email",
                                required: true,
                                class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white placeholder-gray-400 focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 transition-all duration-300",
                                placeholder: "Enter your email"
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
                                class: "h-4 w-4 text-emerald-500 focus:ring-emerald-500 border-gray-400 rounded bg-gray-700"
                            }
                            label {
                                r#for: "remember-me",
                                class: "ml-2 block text-sm text-gray-400",
                                "Remember me"
                            }
                        }
                        div {
                            a {
                                href: "/forgot-password",
                                class: "text-sm text-emerald-400 hover:text-emerald-300 transition-colors hover:underline",
                                "Forgot password?"
                            }
                        }
                    }

                    // Submit Button
                    button {
                        r#type: "submit",
                        class: "w-full bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-semibold py-3 px-6 rounded-xl transition-all duration-300 focus:ring-2 focus:ring-emerald-500 focus:outline-none shadow-lg hover:shadow-xl transform hover:scale-[1.02]",
                        "Sign in"
                    }
                }

                // Register Link
                div {
                    class: "text-center pt-4 border-t border-gray-700",
                    p {
                        class: "text-gray-400 text-sm",
                        "New to KyleHub? "
                        a {
                            href: "/register",
                            class: "text-emerald-400 hover:text-emerald-300 transition-colors hover:underline font-medium",
                            "Create an account"
                        }
                    }
                }
            }
        }
    }
}
