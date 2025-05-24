use sqlx::{Executor, Pool, Sqlite};

use tokio::sync::OnceCell;

use super::model::UserSql;

static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

async fn db() -> Pool<Sqlite> {
    let pool = sqlx::sqlite::SqlitePool::connect("sqlite://db.sqlite")
        .await
        .unwrap();

    pool.execute(
        "
    CREATE TABLE IF NOT EXISTS users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      username TEXT,
      password TEXT
    )
  ",
    )
    .await
    .unwrap();

    let rows: Vec<UserSql> = sqlx::query_as("SELECT * FROM users WHERE id = ?1")
        .bind(&1)
        .fetch_all(&pool)
        .await
        .unwrap();

    if rows.len() == 0 {
        sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind(&"guest")
            .bind(&"guest")
            .execute(&pool)
            .await
            .unwrap();
    }

    pool
}

pub async fn get_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(db).await
}
