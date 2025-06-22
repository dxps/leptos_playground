use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::PgPool;

use crate::{
    domain::model::{Id, UserAccount},
    server::UsersRepo,
};

pub type AuthSession = axum_session_auth::AuthSession<UserAccount, Id, SessionPgPool, PgPool>;
pub type AuthSessionLayer =
    axum_session_auth::AuthSessionLayer<UserAccount, Id, SessionPgPool, PgPool>;

#[async_trait]
impl Authentication<UserAccount, Id, PgPool> for UserAccount {
    //
    async fn load_user(user_id: Id, pool: Option<&PgPool>) -> Result<UserAccount, anyhow::Error> {
        let pool = pool.unwrap();
        UsersRepo::get_by_id(&user_id, pool).await.map_or_else(
            || Err(anyhow::anyhow!("Could not load user")),
            |v| {
                log::debug!("Loaded user: {:#?}", v);
                Ok(v)
            },
        )
    }

    fn is_authenticated(&self) -> bool {
        !self.is_anonymous
    }

    fn is_active(&self) -> bool {
        !self.is_anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSession layer was not found!")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSession layer was not found!",
        )
            .into_response()
    }
}
