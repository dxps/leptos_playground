#[cfg(feature = "ssr")]
#[allow(unused)]
mod ssr_imports {
    pub use axum::{
        Router,
        body::Body as AxumBody,
        extract::{Path, State},
        http::Request,
        response::{Html, IntoResponse, Response},
        routing::{get, post},
    };
    pub use leptos::prelude::*;
    pub use leptos_axum::{LeptosRoutes, generate_route_list};
    pub use user_dir_lap::{server::file_or_index_handler, server_fns_todo::*, *};
}

#[cfg(feature = "ssr")]
#[cfg_attr(feature = "ssr", tokio::main)]
async fn main() {
    use axum::Extension;
    use axum_session::{SessionConfig, SessionLayer};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use axum_session_sqlx::{SessionPgPool, SessionPgSessionStore};
    use sqlx::PgPool;
    use ssr_imports::*;
    use std::sync::Arc;
    use user_dir_lap::{
        domain::model::{Id, UserAccount},
        server::{ServerState, init_logging},
    };

    init_logging();
    dotenvy::dotenv().unwrap();

    log::info!("Connecting to database ...");
    let dbcp = server::db_pool_init()
        .await
        .expect("Failed to connect to the database!");
    log::info!("Connected to database.");
    let session_config = SessionConfig::default().with_table_name("user_sessions");
    let session_store = SessionPgSessionStore::new(Some(dbcp.clone().into()), session_config)
        .await
        .unwrap();
    let auth_config = AuthConfig::<Id>::default().with_anonymous_user_id(Some("iH26rJ8Cp".into()));

    let state = ServerState::new(Arc::new(dbcp.clone()));

    // Setting this to None means we'll be using cargo-leptos and its env vars.
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let app = Router::new()
        // The server function handlers are normally set up by `.leptos_routes()`.
        // Here, we're not actually doing server side rendering, but setting up
        // a manual handler for the server fns.
        // This should include a get() handler if we have any GetUrl-based server fns.
        .route("/api/{*fn_name}", post(leptos_axum::handle_server_fns))
        .fallback(file_or_index_handler)
        .with_state(leptos_options)
        .layer(
            AuthSessionLayer::<UserAccount, Id, SessionPgPool, PgPool>::new(Some(dbcp))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store));

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
