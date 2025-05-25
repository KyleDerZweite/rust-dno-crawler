#![allow(non_snake_case)]
pub(crate) mod error_404;
pub(crate) mod dashboard;
pub(crate) mod login;
pub(crate) mod register;
pub(crate) mod privacy;
pub(crate) mod terms;
pub(crate) mod contact;
pub(crate) mod user_management;

use dioxus::prelude::*;

use {
    error_404::Error404,
    dashboard::Dashboard,
    login::Login,
    register::Register,
    privacy::Privacy,
    terms::Terms,
    contact::Contact,
    user_management::UserManagement,
};

use crate::website::app::AppLayout;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/user-management")]
    UserManagement {},
    #[route("/privacy")]
    Privacy {},
    #[route("/terms")]
    Terms {},
    #[route("/contact")]
    Contact {},
    #[route("/:..route")]
    Error404 { route: Vec<String> },
}

// Route components
#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();

    // Redirect to login by default
    use_effect(move || {
        navigator.push(Route::Login {});
    });

    rsx! {
        div { "Redirecting to login..." }
    }
}