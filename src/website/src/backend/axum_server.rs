use dioxus::prelude::*;

#[cfg(feature="server")]
use super::{auth_session::auth_session_config, db::get_db, router::router, session::session};


#[cfg(feature="server")]
pub async fn launch_server(app: fn()-> Element) {
  let pool = get_db().await;
  let session_store = session().await;
  let auth_session_config = auth_session_config();
  let router =router(session_store, auth_session_config, pool.clone(), app);

  let addr = dioxus::cli_config::fullstack_address_or_localhost();

  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

  axum::serve(listener, router).await.unwrap()
}