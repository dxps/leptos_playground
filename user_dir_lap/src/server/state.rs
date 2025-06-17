use std::fmt::Display;
#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[cfg(feature = "ssr")]
use axum::extract::{FromRef, FromRequestParts};

#[cfg(feature = "ssr")]
use http::{StatusCode, request::Parts};

#[cfg(feature = "ssr")]
#[derive(FromRef, Clone, Debug)]
pub struct ServerState {
    pub db_pool: Option<Arc<PgPool>>,
}

impl ServerState {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        //
        Self {
            db_pool: Some(db_pool),
        }
    }
}

impl Display for ServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServerState{:?}", self.db_pool)
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
        let state = Self::from_ref(state);
        log::debug!("[from_request_parts] server state: {state}");
        Ok(state)
    }
}

// #[cfg(feature = "ssr")]
// impl FromRef<()> for ServerState {
//     fn from_ref(_input: &()) -> Self {
//         Self { db_pool: None }
//     }
// }
