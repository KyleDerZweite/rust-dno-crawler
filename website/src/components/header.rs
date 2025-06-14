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
        header { class: "fixed top-4 left-1/2 transform -translate-x-1/2 z-50 w-full max-w-6xl px-4",
            nav { 
                class: "bg-black/20 backdrop-blur-lg border border-white/10 rounded-xl px-6 py-4 shadow-xl",
                div { class: "flex items-center justify-between",
                    
                    // Logo with gradient
                    div { class: "flex items-center space-x-4",
                        div {
                            class: "w-10 h-10 bg-gradient-to-br from-emerald-500 to-emerald-600 rounded-xl flex items-center justify-center shadow-lg",
                            span { class: "text-white font-bold text-lg", "K" }
                        }
                        h1 { class: "text-xl font-bold text-white tracking-wide", "DNO Crawler" }
                    }
                    
                    // Navigation with better spacing and styling
                    div { class: "flex items-center space-x-3",
                        if props.is_authenticated {
                            a { 
                                href: "/dashboard", 
                                class: "text-white/80 hover:text-emerald-400 transition-all duration-300 px-3 py-2 rounded-lg hover:bg-white/10",
                                "Dashboard" 
                            }
                            if props.user_role.as_ref().map_or(false, |role| role == "admin") {
                                a { 
                                    href: "/user-management", 
                                    class: "text-white/80 hover:text-amber-400 transition-all duration-300 px-3 py-2 rounded-lg hover:bg-white/10",
                                    "Admin" 
                                }
                            }
                            form { 
                                method: "POST", 
                                action: "/logout", 
                                class: "inline",
                                button { 
                                    class: "text-white/80 hover:text-red-400 transition-all duration-300 px-3 py-2 rounded-lg hover:bg-white/10",
                                    "Logout" 
                                }
                            }
                        } else {
                            a { 
                                href: "/login", 
                                class: "text-white/80 hover:text-emerald-400 transition-all duration-300 px-3 py-2 rounded-lg hover:bg-white/10",
                                "Login" 
                            }
                            a { 
                                href: "/register", 
                                class: "bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-medium py-2 px-6 rounded-xl transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-105",
                                "Register" 
                            }
                        }
                    }
                }
            }
        }
    }
}