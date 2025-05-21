use dioxus::prelude::*;

use crate::components::{register::Register, login::Login, user::User, home::Home};


#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/register")]
    Register {},
    #[route("/login")]
    Login{},
    #[route("/user")]
    User {}
}