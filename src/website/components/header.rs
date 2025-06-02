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
    let mut is_mobile_menu_open = use_signal(|| false);

    // Natural theme-aware classes for floating island (simplified to dark theme)
    let island_bg = "bg-neutral-900/80 backdrop-blur-lg border border-stone-700/40 shadow-2xl shadow-black/20";
    let island_hover_bg = "hover:bg-neutral-800/90";
    let text_primary = "text-stone-100";
    let text_secondary = "text-stone-400";

    rsx! {
        // Floating Island Navigation
        header {
            class: "fixed top-4 left-1/2 min-w-[50vw] max-w-6xl transform -translate-x-1/2 z-50 animate-gentle-float",
            nav {
                class: format!(
                    "{} rounded-4xl px-6 py-3 transition-all duration-500 ease-out hover:scale-[1.02]",
                    island_bg
                ),
                div {
                    class: "flex items-center justify-between w-full",

                    // Left Side - Organic Logo Section
                    div {
                        class: "flex items-center space-x-3 group flex-shrink-0",
                        div {
                            class: "flex items-center space-x-3 transition-all duration-300",
                            // Living logo that "grows"
                            div {
                                class: "w-8 h-8 bg-green-500 rounded-lg flex items-center justify-center group-hover:bg-green-600 transition-colors",
                                span { class: "text-white font-bold text-sm z-10", "K" }
                            }
                            div {
                                class: "hidden md:block",
                                h1 {
                                    class: format!(
                                        "text-lg font-semibold {} group-hover:text-moss-600 transition-all duration-300 tracking-wide",
                                        text_primary
                                    ),
                                    "DNO Crawler"
                                }
                            }
                        }
                    }

                    // Center - Navigation Items (Desktop)
                    if props.is_authenticated {
                        div {
                            class: "hidden lg:flex items-center space-x-2 flex-1 justify-center",

                            // Dashboard - Leaf icon
                            a {
                                href: "/dashboard",
                                class: format!(
                                    "flex items-center space-x-2 px-4 py-2 rounded-2xl {} {} transition-all duration-300 group",
                                    text_primary, island_hover_bg
                                ),
                                title: "Dashboard",
                                svg {
                                    class: "w-5 h-5 group-hover:scale-110 transition-transform duration-300",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
                                    }
                                }
                                span { class: "hidden xl:block text-sm font-medium", "Dashboard" }
                            }

                            if let Some(role) = &props.user_role {
                                if role == "admin" {
                                    // Admin - Tree icon
                                    a {
                                        href: "/user-management",
                                        class: format!(
                                            "flex items-center space-x-2 px-4 py-2 rounded-2xl {} {} transition-all duration-300 group hover:text-amber-600",
                                            text_primary, island_hover_bg
                                        ),
                                        title: "Admin Panel",
                                        svg {
                                            class: "w-5 h-5 group-hover:scale-110 transition-transform duration-300",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"
                                            }
                                        }
                                        span { class: "hidden xl:block text-sm font-medium", "Admin" }
                                    }
                                }
                            }
                        }
                    } else {
                        // Empty spacer when not authenticated to push action items to the right
                        div { class: "hidden lg:block flex-1" }
                    }

                    // Right Side - Action Items
                    div {
                        class: "flex items-center space-x-2 flex-shrink-0",

                        if props.is_authenticated {
                            // Logout - Natural flowing icon
                            form {
                                method: "POST",
                                action: "/logout",
                                class: "inline",
                                button {
                                    r#type: "submit",
                                    class: format!(
                                        "p-2 rounded-2xl {} {} transition-all duration-300 group hover:scale-110 hover:text-red-500",
                                        text_primary, island_hover_bg
                                    ),
                                    title: "Logout",
                                    svg {
                                        class: "w-5 h-5 group-hover:-translate-x-1 transition-transform duration-300",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                                        }
                                    }
                                }
                            }
                        } else {
                            // Login - Growing leaf
                            a {
                                href: "/login",
                                class: format!(
                                    "flex items-center space-x-2 px-4 py-2 rounded-2xl {} {} transition-all duration-300 group hover:text-moss-600",
                                    text_primary, island_hover_bg
                                ),
                                svg {
                                    class: "w-5 h-5 group-hover:scale-110 transition-transform duration-300",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"
                                    }
                                }
                                span { class: "hidden sm:block text-sm font-medium", "Login" }
                            }

                            // Register - Blooming button
                            a {
                                href: "/register",
                                class: format!(
                                    "flex items-center space-x-2 px-4 py-2 bg-gradient-to-r from-moss-500 to-moss-600 {text_primary} rounded-2xl text-sm font-medium transition-all duration-300 hover:animate-grow-bloom group"
                                ),
                                svg {
                                    class: "w-4 h-4 group-hover:scale-110 transition-transform duration-300",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 6v6m0 0v6m0-6h6m-6 0H6"
                                    }
                                }
                                span { class: "hidden sm:block", "Register" }
                            }
                        }

                        // Mobile Menu Toggle
                        button {
                            class: format!(
                                "lg:hidden p-2 rounded-2xl {} {} transition-all duration-300 group",
                                text_primary, island_hover_bg
                            ),
                            onclick: move |_| {
                                is_mobile_menu_open.set(!is_mobile_menu_open());
                            },
                            svg {
                                class: "w-5 h-5 group-hover:scale-110 transition-transform duration-300",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                if is_mobile_menu_open() {
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12"
                                    }
                                } else {
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
            }
        }

        // Mobile Organic Dropdown
        if is_mobile_menu_open() {
            div {
                class: format!(
                    "fixed top-20 left-1/2 transform -translate-x-1/2 z-40 {} rounded-2xl p-6 min-w-[280px] lg:hidden animate-bloom-in",
                    island_bg
                ),
                div {
                    class: "space-y-3",
                    if props.is_authenticated {
                        a {
                            href: "/dashboard",
                            class: format!(
                                "flex items-center space-x-3 p-3 rounded-2xl {} {} transition-all duration-300 w-full",
                                text_primary, island_hover_bg
                            ),
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
                                }
                            }
                            span { "Dashboard" }
                        }
                        if let Some(role) = &props.user_role {
                            if role == "admin" {
                                a {
                                    href: "/user-management",
                                    class: format!(
                                        "flex items-center space-x-3 p-3 rounded-2xl {} {} transition-all duration-300 w-full",
                                        text_primary, island_hover_bg
                                    ),
                                    svg {
                                        class: "w-5 h-5",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"
                                        }
                                    }
                                    span { "Admin Panel" }
                                }
                            }
                        }
                    }

                    div { class: format!("border-t {} my-3", "border-stone-700") }

                    if !props.is_authenticated {
                        a {
                            href: "/login",
                            class: format!(
                                "flex items-center space-x-3 p-3 rounded-2xl {} {} transition-all duration-300 w-full",
                                text_primary, island_hover_bg
                            ),
                            span { "Login" }
                        }
                        a {
                            href: "/register",
                            class: "flex items-center justify-center space-x-2 p-3 bg-gradient-to-r from-moss-500 to-moss-600 {text_primary} rounded-2xl transition-all duration-300 w-full hover:animate-grow-bloom",
                            span { "Register" }
                        }
                    } else {
                        form {
                            method: "POST",
                            action: "/logout",
                            class: "w-full",
                            button {
                                r#type: "submit",
                                class: format!(
                                    "flex items-center space-x-3 p-3 rounded-2xl {} {} transition-all duration-300 w-full text-red-500 hover:text-red-600",
                                    text_primary, island_hover_bg
                                ),
                                span { "Logout" }
                            }
                        }
                    }
                }
            }
        }
    }
}
