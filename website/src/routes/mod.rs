pub mod error_404;
pub mod dashboard;
pub mod home;
pub mod login;
pub mod register;
pub mod privacy;
pub mod terms;
pub mod contact;
pub mod user_management;
pub mod impressum;
pub mod demo;

use dioxus::prelude::*;
use crate::layout::Layout;

pub use error_404::Error404;
pub use dashboard::Dashboard;
pub use home::Home;
pub use login::Login;
pub use register::Register;
pub use privacy::Privacy;
pub use terms::Terms;
pub use contact::Contact;
pub use impressum::Impressum;
pub use demo::Demo;

/// Route configuration for the website
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/privacy")]
    Privacy {},
    #[route("/terms")]
    Terms {},
    #[route("/contact")]
    Contact {},
    #[route("/impressum")]
    Impressum {},
    #[route("/demo")]
    Demo {},
    #[route("/:..route")]
    Error404 { route: Vec<String> },
}
