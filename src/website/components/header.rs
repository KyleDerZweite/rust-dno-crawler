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
    rsx! {
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
                    div { class: "flex items-center space-x-4",
                        if props.is_authenticated {
                            a {
                                href: "/dashboard",
                                class: "text-gray-700 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Dashboard"
                            }
                            if let Some(role) = &props.user_role {
                                if role == "admin" {
                                    a {
                                        href: "/user-management",
                                        class: "text-gray-700 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium",
                                        "User Management"
                                    }
                                }
                            }
                            form { method: "POST", action: "/logout", class: "inline",
                                button {
                                    r#type: "submit",
                                    class: "text-gray-700 hover:text-red-600 px-3 py-2 rounded-md text-sm font-medium",
                                    "Logout"
                                }
                            }
                        } else {
                            a {
                                href: "/login",
                                class: "text-gray-700 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium",
                                "Login"
                            }
                            a {
                                href: "/register",
                                class: "bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-2 rounded-md text-sm font-medium",
                                "Sign Up"
                            }
                        }
                    }
                }
            }
        }
    }
}