
use axum_session::SessionSqlitePool;

use axum_session_auth::{AuthConfig, AuthSession, Authentication};

use sqlx::SqlitePool;

use async_trait::async_trait;

use dioxus::prelude::*;


use super::model::UserSql;


pub fn auth_session_config () -> AuthConfig<i64> {
  AuthConfig::<i64>::default().with_anonymous_user_id(Some(1))
}

#[derive(Clone)]
pub struct User {
  pub id : i64,
  pub anonymous: bool,
  pub username: String
}


#[async_trait]
impl Authentication<User, i64, SqlitePool> for  User{
    async fn load_user(userid:i64,pool:Option< & SqlitePool>) -> Result<User, anyhow::Error> {
        if userid == 1 {
          Ok(User { id: userid, anonymous: true, username: String::from("guest") })
        } else {
          let user: UserSql = sqlx::query_as("SELECT * FROM users WHERE id = ?1").bind(&userid).fetch_one(pool.unwrap()).await.unwrap();
          Ok(User { id: userid, anonymous: false, username: user.username })
        }
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }
}


type AuthSessionExtract = AuthSession<User, i64, SessionSqlitePool, SqlitePool>;


pub async fn get_auth_session() -> Result<AuthSessionExtract, ServerFnError> {
  extract::<AuthSessionExtract, _>().await.map_err(|_| ServerFnError::new("Auth session not Found!"))
}