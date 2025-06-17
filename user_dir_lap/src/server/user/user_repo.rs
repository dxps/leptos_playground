use sqlx::{FromRow, PgPool, Row, postgres::PgRow};
use std::sync::Arc;

use crate::{
    domain::model::{Id, UserAccount, UserEntry},
    errors::{AppError, AppResult, AppUseCase},
};

#[derive(Debug)]
pub struct UsersRepo {
    dbcp: Arc<PgPool>,
}

impl UsersRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get_by_email(&self, email: &String, usecase: AppUseCase) -> AppResult<UserEntry> {
        //
        sqlx::query_as::<_, UserEntry>(
            "SELECT id, email, username, password, salt, bio, is_anonymous FROM user_accounts 
             WHERE email = $1",
        )
        .bind(email)
        .fetch_one(self.dbcp.as_ref())
        .await
        .map_err(|err| AppError::from((err, usecase)))
    }
}

// ///////////////////////////////////////////
// Implementations of sqlx's FromRow trait. //
// ///////////////////////////////////////////

impl FromRow<'_, PgRow> for UserEntry {
    //
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user: UserAccount {
                id: Id::new_from(row.try_get("id").unwrap_or_default()),
                email: row.get("email"),
                username: row.get("username"),
                bio: row.get("bio"),
                is_anonymous: row.get("is_anonymous"),
                permissions: Vec::new(),
            },
            password: row.get("password"),
            salt: row.get("salt"),
        })
    }
}
