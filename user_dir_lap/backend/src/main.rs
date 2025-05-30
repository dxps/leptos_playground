use argon2::{self, Config};
use axum::{Router, routing::post};
use leptos::*;
use leptos_axum::LeptosRoutes;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tower_cookies::{CookieManagerLayer, Cookies};
use uuid::Uuid;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    message: String,
}

async fn login_handler(
    cookies: Cookies,
    pool: axum::extract::Extension<PgPool>,
    axum::Json(payload): axum::Json<LoginRequest>,
) -> axum::Json<LoginResponse> {
    let row = sqlx::query!(
        "SELECT password_hash FROM users WHERE username = $1",
        payload.username
    )
    .fetch_optional(&**pool)
    .await;

    match row {
        Ok(Some(record)) => {
            if argon2::verify_encoded(&record.password_hash, payload.password.as_bytes())
                .unwrap_or(false)
            {
                let session_id = Uuid::new_v4().to_string();
                cookies.add(
                    axum::http::HeaderValue::from_str(&format!(
                        "session={}; Path=/; HttpOnly",
                        session_id
                    ))
                    .unwrap(),
                );
                axum::Json(LoginResponse {
                    success: true,
                    message: "Login successful".into(),
                })
            } else {
                axum::Json(LoginResponse {
                    success: false,
                    message: "Invalid password".into(),
                })
            }
        }
        Ok(None) => axum::Json(LoginResponse {
            success: false,
            message: "User not found".into(),
        }),
        Err(_) => axum::Json(LoginResponse {
            success: false,
            message: "Database error".into(),
        }),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/api/login", post(login_handler))
        .layer(CookieManagerLayer::new())
        .layer(axum::extract::Extension(pool));

    leptos_axum::run_leptos(app).await;
}
