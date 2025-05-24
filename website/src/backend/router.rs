
use axum::Router;

use axum_session::{SessionLayer, SessionSqlitePool, SessionStore};

use axum_session_auth::{AuthConfig, AuthSessionLayer};
use dioxus::prelude::*;

use sqlx::{Pool, Sqlite, SqlitePool};

use super::auth_session::User;


pub fn router(session_store : SessionStore<SessionSqlitePool>, auth_config: AuthConfig<i64>, pool: Pool<Sqlite>, app: fn() -> Element) -> Router {
  let config = ServeConfig::new().unwrap();

  Router::new()
    .serve_dioxus_application(config, app)
    .layer(AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool)).with_config(auth_config))
    .layer(SessionLayer::new(session_store))
}