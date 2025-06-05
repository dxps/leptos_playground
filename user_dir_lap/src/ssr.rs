// use http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode};
use leptos::server_fn::ServerFnError;
// use sqlx::{Connection, SqliteConnection};
use crate::errors::AppError;
use sqlx::{PgPool, postgres::PgPoolOptions};

// pub async fn db_pool_init() -> Result<SqliteConnection, ServerFnError> {
//     Ok(SqliteConnection::connect("sqlite:iam.db").await?)
// }
pub async fn db_pool_init() -> Result<PgPool, ServerFnError> {
    //
    let db_url = std::env::var("DATABASE_URL").map_err(|err| {
        log::error!(
            "Unknown DATABASE_URL environment variable. Reason: '{}'.",
            err
        );
        AppError::Err("Unknown DATABASE_URL environment variable".into())
    })?;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(db_url.as_str())
        .await
        .map_err(|_| AppError::Err("Failed to connect to database".into()))?;
    log::info!("Connected to database.");
    Ok(pool)
}
