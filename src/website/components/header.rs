#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    #[props(default = None)]
    pub user_role: Option<String>,
    #[props(default = false)]
    pub is_authenticated: bool,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    // State für Mobile Menu Toggle
    let mut is_mobile_menu_open = use_signal(|| false);

    rsx! {
        header {
            class: "nav sticky top-0 z-50 backdrop-blur-sm bg-neutral-900/95 border-b border-neutral-700",
            nav {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div {
                    class: "flex justify-between items-center h-16",

                    // Logo/Brand Section
                    div {
                        class: "flex items-center flex-shrink-0",
                        a {
                            href: "/",
                            class: "flex items-center space-x-3 group",
                            // Logo Icon (grüner Punkt als Akzent)
                            div {
                                class: "w-8 h-8 bg-green-500 rounded-lg flex items-center justify-center group-hover:bg-green-600 transition-colors",
                                span { class: "text-white font-bold text-sm", "K" }
                            }
                            h1 {
                                class: "text-xl font-bold text-neutral-100 group-hover:text-green-500 transition-colors hidden sm:block",
                                "DNO - Crawler"
                            }
                        }
                    }

                    // Desktop Navigation
                    div {
                        class: "hidden md:flex items-center space-x-1",
                        if props.is_authenticated {
                            a {
                                href: "/dashboard",
                                class: "text-neutral-400 hover:text-green-500 hover:bg-neutral-800 px-3 py-2 rounded-xl text-sm font-medium transition-all duration-200",
                                "Dashboard"
                            }
                            if let Some(role) = &props.user_role {
                                if role == "admin" {
                                    a {
                                        href: "/user-management",
                                        class: "text-neutral-400 hover:text-amber-600 hover:bg-neutral-800 px-3 py-2 rounded-xl text-sm font-medium transition-all duration-200",
                                        "Admin"
                                    }
                                }
                            }
                            // Theme Toggle Button
                            button {
                                class: "theme-toggle",
                                onclick: move |_| {
                                    // Theme toggle logic hier implementieren
                                },
                                svg {
                                    class: "w-5 h-5",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    path { d: "M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" }
                                }
                            }
                            // Logout Button
                            form {
                                method: "POST",
                                action: "/logout",
                                class: "inline",
                                button {
                                    r#type: "submit",
                                    class: "text-neutral-400 hover:text-red-400 hover:bg-neutral-800 px-3 py-2 rounded-xl text-sm font-medium transition-all duration-200",
                                    "Logout"
                                }
                            }
                        } else {
                            a {
                                href: "/login",
                                class: "text-neutral-400 hover:text-green-500 hover:bg-neutral-800 px-3 py-2 rounded-xl text-sm font-medium transition-all duration-200",
                                "Login"
                            }
                            a {
                                href: "/register",
                                class: "btn-primary ml-2",
                                "Register"
                            }
                        }
                    }

                    // Mobile Menu Button
                    div {
                        class: "flex md:hidden",
                        button {
                            class: "text-neutral-400 hover:text-neutral-100 hover:bg-neutral-800 p-2 rounded-lg transition-colors",
                            onclick: move |_| {
                                is_mobile_menu_open.set(!is_mobile_menu_open());
                            },
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                if is_mobile_menu_open() {
                                    // X Icon für Close
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12"
                                    }
                                } else {
                                    // Hamburger Icon
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M4 6h16M4 12h16M4 18h16"
                                    }
                                }
                            }
                        }
                    }
                }

                // Mobile Menu (nur sichtbar wenn geöffnet)
                if is_mobile_menu_open() {
                    div {
                        class: "md:hidden border-t border-neutral-700 bg-neutral-800/95 backdrop-blur-sm",
                        div {
                            class: "px-2 pt-2 pb-3 space-y-1",
                            if props.is_authenticated {
                                a {
                                    href: "/dashboard",
                                    class: "block text-neutral-400 hover:text-green-500 hover:bg-neutral-700 px-3 py-2 rounded-xl text-base font-medium transition-all",
                                    "Dashboard"
                                }
                                if let Some(role) = &props.user_role {
                                    if role == "admin" {
                                        a {
                                            href: "/user-management",
                                            class: "block text-neutral-400 hover:text-amber-600 hover:bg-neutral-700 px-3 py-2 rounded-xl text-base font-medium transition-all",
                                            "Admin Panel"
                                        }
                                    }
                                }
                                // Mobile Theme Toggle
                                button {
                                    class: "w-full text-left text-neutral-400 hover:text-neutral-100 hover:bg-neutral-700 px-3 py-2 rounded-xl text-base font-medium transition-all",
                                    "Toggle Theme"
                                }
                                // Mobile Logout
                                form {
                                    method: "POST",
                                    action: "/logout",
                                    class: "block",
                                    button {
                                        r#type: "submit",
                                        class: "w-full text-left text-neutral-400 hover:text-red-400 hover:bg-neutral-700 px-3 py-2 rounded-xl text-base font-medium transition-all",
                                        "Logout"
                                    }
                                }
                            } else {
                                a {
                                    href: "/login",
                                    class: "block text-neutral-400 hover:text-green-500 hover:bg-neutral-700 px-3 py-2 rounded-xl text-base font-medium transition-all",
                                    "Login"
                                }
                                a {
                                    href: "/register",
                                    class: "block btn-primary mt-2 text-center",
                                    "Sign Up"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}