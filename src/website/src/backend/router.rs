#[cfg(feature="server")]
use axum::Router;
#[cfg(feature="server")]
use axum_session::{SessionLayer, SessionSqlitePool, SessionStore};
#[cfg(feature="server")]
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use dioxus::prelude::*;
#[cfg(feature="server")]
use sqlx::{Pool, Sqlite, SqlitePool};
#[cfg(feature="server")]
use super::auth_session::User;

#[cfg(feature="server")]
pub fn router(session_store : SessionStore<SessionSqlitePool>, auth_config: AuthConfig<i64>, pool: Pool<Sqlite>, app: fn() -> Element) -> Router {
  let config = ServeConfig::new().unwrap();

  Router::new()
    .serve_dioxus_application(config, app)
    .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool)).with_config(auth_config))
    .layer(SessionLayer::new(session_store))
}