use std::sync::Arc;

use randoid::randoid;

use crate::{
    domain::model::UserAccount,
    errors::{AppError, AppResult, AppUseCase},
    server::UsersRepo,
};

#[derive(Clone, Debug)]
pub struct UserMgmt {
    user_repo: Arc<UsersRepo>,
}

impl UserMgmt {
    //
    pub fn new(user_repo: Arc<UsersRepo>) -> Self {
        Self { user_repo }
    }

    pub async fn authenticate_user(&self, email: String, pwd: String) -> AppResult<UserAccount> {
        //
        let user_entry = self
            .user_repo
            .get_by_email(&email, AppUseCase::UserLogin)
            .await?;
        match Self::check_password(&pwd, &user_entry.password, &user_entry.salt) {
            true => Ok(user_entry.into()),
            false => Err(AppError::Unauthorized("wrong credentials".into())),
        }
    }

    fn generate_password(pwd: String) -> (String, String) {
        //
        // let salt: String = std::iter::repeat_with(fastrand::alphanumeric)
        // .take(12)
        // .collect();

        let salt = randoid!(12);
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        (format!("{:x}", digest), salt)
    }

    fn regenerate_password(pwd: String, salt: String) -> String {
        //
        let digest = md5::compute(format!("@{salt}${pwd}").as_bytes());
        format!("{:x}", digest)
    }

    fn check_password(input_pwd: &str, pwd: &str, salt: &str) -> bool {
        //
        let digest = md5::compute(format!("@{salt}${input_pwd}").as_bytes());
        pwd == format!("{:x}", digest)
    }
}
