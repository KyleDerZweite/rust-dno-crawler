
#[cfg(feature="server")]
#[derive(sqlx::FromRow)]
pub struct UserSql {
  pub id : i64,
  pub username: String,
  pub password: String
}