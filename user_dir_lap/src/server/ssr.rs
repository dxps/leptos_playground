use crate::errors::AppError;
use leptos::server_fn::ServerFnError;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn db_pool_init() -> Result<PgPool, ServerFnError> {
    //
    let db_url = std::env::var("DATABASE_URL").map_err(|err| {
        log::error!(
            "DATABASE_URL environment variable is not set. Reason: '{}'.",
            err
        );
        AppError::Err("DATABASE_URL environment variable is not set.".into())
    })?;
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(db_url.as_str())
        .await
        .map_err(|_| AppError::Err("Failed to connect to database".into()))?;

    Ok(pool)
}
