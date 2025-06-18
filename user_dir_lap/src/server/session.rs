use std::sync::Arc;

#[cfg(feature = "ssr")]
use axum::extract::FromRequestParts;
#[cfg(feature = "ssr")]
use http::request::Parts;

#[cfg(feature = "ssr")]
use crate::server::AuthSessionLayerNotFound;
use crate::server::{AuthSession, UserMgmt};

#[derive(Debug)]
pub struct Session {
    //
    pub user_mgmt: Arc<UserMgmt>,
    pub auth_session: AuthSession,
}

#[cfg(feature = "ssr")]
impl<S: Send + Sync> FromRequestParts<S> for Session {
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        //
        use crate::server::AuthSession;
        AuthSession::from_request_parts(parts, state)
            .await
            .map(|auth_session| {
                use crate::server::ServerState;

                let server_state = parts.extensions.get::<ServerState>().unwrap();
                log::debug!("[from_request_parts] server_state: {server_state:?}");
                Session {
                    user_mgmt: server_state.user_mgmt.clone(),
                    auth_session,
                }
            })
            .map_err(|_| AuthSessionLayerNotFound)
    }
}
