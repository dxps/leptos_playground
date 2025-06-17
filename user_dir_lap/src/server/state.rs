#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg(feature = "ssr")]
use axum::extract::{FromRef, FromRequestParts};

#[cfg(feature = "ssr")]
use http::{StatusCode, request::Parts};

use crate::server::{UserMgmt, UsersRepo};

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct ServerState {
    pub user_mgmt: Arc<UserMgmt>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        //
        let user_mgmt = Arc::new(UserMgmt::new(Arc::new(UsersRepo::new(db_pool.clone()))));
        Self { user_mgmt }
    }
}

#[cfg(feature = "ssr")]
impl<S> FromRequestParts<S> for ServerState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
