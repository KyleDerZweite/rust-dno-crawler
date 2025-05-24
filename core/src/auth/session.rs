
use axum_session::{Key, SessionConfig, SessionSqlitePool, SessionStore};

use super::db::get_db;

pub async fn session() -> SessionStore<SessionSqlitePool> {
  let pool = get_db().await;
  let config = SessionConfig::new().with_table_name("session_table").with_key(Key::generate());

  SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), config).await.unwrap()
}