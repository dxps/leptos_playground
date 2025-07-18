#[cfg(feature = "ssr")]
#[allow(non_snake_case)]
mod ssr_imports {
    pub use axum::{
        Router,
        routing::{get, post},
    };
    pub use leptos::prelude::*;
    pub use user_dir_lap::{server::file_or_index_handler, *};
}

#[cfg(feature = "ssr")]
#[cfg_attr(feature = "ssr", tokio::main)]
async fn main() {
    use std::sync::Arc;

    use axum::Extension;
    use axum_session::{SessionConfig, SessionLayer};
    use axum_session_auth::AuthConfig;
    use axum_session_sqlx::SessionPgSessionStore;
    use ssr_imports::*;
    use user_dir_lap::{
        domain::model::Id,
        server::{AuthSessionLayer, ServerState, init_logging},
    };

    init_logging();
    dotenvy::dotenv().unwrap();

    log::info!("Connecting to database ...");
    let dbcp = server::db_pool_init()
        .await
        .expect("Failed to connect to the database!");
    log::info!("Connected to database.");

    let session_config = SessionConfig::default()
        .with_session_name("user_dir_lap_session")
        .with_table_name("user_sessions");
    let session_store = SessionPgSessionStore::new(Some(dbcp.clone().into()), session_config)
        .await
        .unwrap();
    let auth_config = AuthConfig::<Id>::default().with_anonymous_user_id(Some("iH26rJ8Cp".into()));

    let state = ServerState::new(Arc::new(dbcp.clone()));

    match state
        .user_mgmt
        .register_admin_user(
            "Admin".into(),
            "admin@example.com".into(),
            "admin".into(),
            "admin".into(),
        )
        .await
    {
        Ok(_) => log::info!("Self-registered the admin user."),
        Err(e) => {
            use user_dir_lap::app_err_uc::AppError;

            if let AppError::AlreadyExists(_) = e {
                // It's fine if the admin user already exists.
            } else {
                log::error!("Failed to self-register the admin user: {}", e);
            }
        }
    }

    // Setting this to None means we'll be using cargo-leptos and its env vars.
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let app = Router::new()
        // The server function handlers are normally set up by `.leptos_routes()`.
        // Here, we're not actually doing server side rendering, but setting up
        // a manual handler for the server fns.
        .route("/api/{*fn_name}", post(leptos_axum::handle_server_fns))
        .route("/api/{*fn_name}", get(leptos_axum::handle_server_fns))
        .fallback(file_or_index_handler)
        .with_state(leptos_options)
        .layer(AuthSessionLayer::new(Some(dbcp)).with_config(auth_config))
        .layer(SessionLayer::new(session_store))
        .layer(Extension(state));

    log::info!("Listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // This example cannot be built as a trunk standalone CSR-only app.
    // Only the server may directly connect to the database.
}
