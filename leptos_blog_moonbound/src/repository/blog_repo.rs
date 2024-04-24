use std::{thread::sleep, time::Duration};

use leptos::{logging::log, server, ServerFnError};

use crate::model::Post;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::ServerFnError;
    use sqlx::{
        sqlite::{SqlitePool, SqlitePoolOptions},
        Connection, SqliteConnection,
    };

    pub async fn db_conn() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:post.db").await?)
    }

    pub async fn db_pool() -> Result<SqlitePool, ServerFnError> {
        Ok(SqlitePoolOptions::new()
            .connect("sqlite:post.db")
            .await
            .expect("Could not make db pool."))
    }
}

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    content: String,
) -> Result<String, ServerFnError> {
    //
    use uuid::Uuid;

    let mut db_conn = self::ssr::db_conn().await?;
    let id = id.unwrap_or(Uuid::new_v4().to_string());
    sqlx::query(
        "INSERT INTO post VALUES ($1, $2, $3, $4, $5)
                 ON CONFLICT (id) DO UPDATE SET dt=excluded.dt,
                 image_url=excluded.image_url,
                 title=excluded.title,
                 text=excluded.text",
    )
    .bind(&id)
    .bind(&dt)
    .bind(&image_url)
    .bind(&title)
    .bind(&content)
    .execute(&mut db_conn)
    .await?;

    Ok(id)
}

#[server(GetPost, "/api", "GetJson")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    //
    log!("[get_post] Serving for id {id}");
    // FYI: Temporarily used for testing the Suspense feature in EditPost component.
    sleep(Duration::from_secs(3));

    let mut db_conn = self::ssr::db_conn().await?;

    match sqlx::query_as::<_, Post>("SELECT * FROM post WHERE id = ?")
        .bind(&id)
        .fetch_one(&mut db_conn)
        .await
    {
        Ok(post) => {
            log!(
                "{}",
                format!("get_post({}) found post w/ title='{}'.", &id, &post.title)
            );
            Ok(post)
        }
        Err(e) => {
            log!("[get_post] Error {}", e.to_string());
            Err(ServerFnError::ServerError(e.to_string()))
        }
    }
}

#[server(DeletePost, "/api")]
pub async fn delete_post(id: String) -> Result<(), ServerFnError> {
    log!("[delete_post] Deleting by id {:?} ...", &id);
    let mut db_conn = self::ssr::db_conn().await?;

    match sqlx::query("DELETE FROM post WHERE ID = ?")
        .bind(id)
        .execute(&mut db_conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            log!("[delete_post] Error {}", e.to_string());
            Err(ServerFnError::ServerError(e.to_string()))
        }
    }
}

#[server(GetPreviews, "/api", "GetJson")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    //
    let mut db_conn = self::ssr::db_conn().await?;
    let res: Vec<Post> = sqlx::query_as(
        "SELECT id, dt, image_url, title
         CASE
            WHEN LENGTH(text) > $1 THEN SUBSTR(text, $1) || ' ...'
         END as text
         FROM posts
         ORDER BY dt DESC
         LIMIT $2",
    )
    .bind(preview_length)
    .bind(page_size)
    .fetch_all(&mut db_conn)
    .await?;

    Ok(res)
}
