use crate::{domain::model::UserAccount, dtos::LoginResult};
use leptos::prelude::*;
use server_fn::codec::{GetUrl, PostUrl};

#[server(endpoint = "login", input = PostUrl)]
pub async fn login(username: String, password: String) -> Result<LoginResult, ServerFnError> {
    //
    use crate::server::Session;

    let sess: Session = leptos_axum::extract().await?;

    let login_res = sess.user_mgmt.authenticate_user(username, password).await;
    if login_res.is_succcess {
        let user_id = login_res.clone().account.unwrap().id;
        log::debug!("Logged in user w/ id: {user_id}");
        sess.auth_session.login_user(user_id);
    }

    Ok(login_res)
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    //
    use crate::server::Session;

    let sess: Session = leptos_axum::extract().await?;

    sess.auth_session.logout_user();
    Ok(())
}

#[server]
pub async fn is_logged_in() -> Result<bool, ServerFnError> {
    //
    use crate::server::Session;

    let sess: Session = leptos_axum::extract().await?;
    Ok(sess.auth_session.is_authenticated())
}

#[server(endpoint = "current_user", input = GetUrl)]
pub async fn get_current_user() -> Result<Option<UserAccount>, ServerFnError> {
    //
    use crate::server::Session;

    let mut sess: Session = leptos_axum::extract().await?;
    let mut curr_user = sess.current_user();
    log::debug!(
        "Current user: {:?} is_authenticated: {}",
        sess.auth_session.current_user,
        sess.auth_session.is_authenticated()
    );

    // Additional investigation.
    if curr_user.is_none() {
        sess.auth_session.reload_user().await;
        curr_user = sess.current_user();
        log::debug!(
            "After reloading, current user: {:?} is_authenticated: {}",
            sess.auth_session.current_user,
            sess.auth_session.is_authenticated()
        );
    }

    Ok(curr_user)
}
