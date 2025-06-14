use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use axum_session_auth::*;
use sqlx::PgPool;

use crate::domain::model::{Id, UserAccount};

#[async_trait]
impl Authentication<UserAccount, Id, PgPool> for UserAccount {
    //

    //async fn load_user(userid: Type, pool: Option<&Pool>) -> Result<User, Error>;
    //Type: Eq + Default + Clone + Send + Sync + Hash + Serialize + DeserializeOwned + 'static,

    async fn load_user(_user_id: Id, pool: Option<&PgPool>) -> Result<UserAccount, anyhow::Error> {
        let _pool = pool.unwrap();
        // UsersRepo::get_by_id(&user_id, pool)
        //     .await
        //     .ok_or_else(|| anyhow::anyhow!("Could not load user"))
        Err(anyhow::anyhow!("Not implemented"))
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
