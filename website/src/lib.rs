#![allow(non_snake_case)]
mod router;

use anyhow::Result;
use backend::axum_server::launch_server;

use crate::router::{home::Home, login::Login, register::Register, user::User};

use dioxus::prelude::*;

pub async fn create_client() -> Result<()> {
    println!("Initializing website...");

    tokio::runtime::Runtime::new()?.block_on(async {
        launch_server(App).await;
    });

    Ok(())
}

#[component]
pub fn App() -> Element {
    rsx!(
        document::Stylesheet { href: asset!("./public/tailwind_output.css") }
        Router::<Route> {}
    )
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/register")]
    Register {},
    #[route("/login")]
    Login {},
    #[route("/user")]
    User {},
}
