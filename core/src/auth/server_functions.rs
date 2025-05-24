use dioxus::prelude::*;


use super::{auth_session:: get_auth_session, db::get_db, model::UserSql};


pub async fn register(username: String, password: String) -> Result<(), ServerFnError> {
  if username.trim() == "" || password.is_empty() {
    let msg = format!("Username or Password can't be empty!");
    Err(ServerFnError::new(msg))
  } else {
      let pool = get_db().await;
      let rows: Vec<UserSql> = sqlx::query_as("SELECT * FROM users WHERE username = ?1").bind(&username).fetch_all(pool).await.unwrap();
      if rows.len() != 0 {
        let msg = format!("Username  {} is already taken!", username);
        Err(ServerFnError::new(msg))
      } else {
        let hash_password = bcrypt::hash(password, 10).unwrap();
        sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)").bind(&username).bind(&hash_password).execute(pool).await.unwrap();
        Ok(())
      }
  }
}

pub async fn log_in(username: String, password: String) -> Result<(), ServerFnError> {
  if username.trim() == "" || password.is_empty() {
    let msg = format!("Username or Password can't be empty!");
    Err(ServerFnError::new(msg))
  } else if username == "guest" {
    let msg = format!("Guest is not allowed to log in.");
    Err(ServerFnError::new(msg))
  } else {
    let pool = get_db().await;
    let rows: Vec<UserSql> = sqlx::query_as("SELECT * FROM users WHERE username = ?1").bind(&username).fetch_all(pool).await.unwrap();

    if rows.len() == 0 {
      let msg = format!("Username {} is not registered!", username);
      Err(ServerFnError::new(msg))
    } else {
      let is_valid = bcrypt::verify(password, &rows[0].password).unwrap();
      if is_valid {
        let auth_session = get_auth_session().await?;
        auth_session.login_user(rows[0].id);
        Ok(())
      } else {
        let msg = format!("Password is not correct!");
        Err(ServerFnError::new(msg))
      }
    }
  }
}

pub async fn log_out () -> Result<(), ServerFnError> {
  let auth_session = get_auth_session().await?;
  auth_session.logout_user();
  Ok(())
}

pub async fn get_user() -> Result<String, ServerFnError> {
  let auth_session = get_auth_session().await?;

  if auth_session.is_authenticated() {
    let user = auth_session.current_user.unwrap();
    let msg = format!("Hello {}, your id is {} !", user.username, user.id);
    Ok(msg)
  } else {
    let msg = format!("You are not Authorizied!"); 
    Err(ServerFnError::new(msg))
  }
}