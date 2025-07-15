use leptos::prelude::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::ops::Range;

#[cfg(feature = "ssr")]
use sqlx::{QueryBuilder, Row};

#[cfg(feature = "ssr")]
use leptos::logging::log;

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[table(sortable, classes_provider = TailwindClassesPreset)]
pub struct UserAccountRow {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAccountQuery {
    #[serde(default)]
    pub sort: VecDeque<(usize, ColumnSort)>,
    pub range: Range<usize>,
    pub name: String,
}

#[server(endpoint = "/accounts", input = server_fn::codec::GetUrl)]
pub async fn list_user_accounts(
    query: UserAccountQuery,
) -> Result<Vec<UserAccountRow>, ServerFnError> {
    //
    use crate::server::get_db_pool;

    let UserAccountQuery { sort, range, name } = query;

    let mut query = QueryBuilder::new("SELECT id, name, email, username FROM user_accounts ");
    if !name.is_empty() {
        query.push("WHERE name LIKE concat('%', ");
        query.push_bind(&name);
        query.push(", '%') OR email LIKE concat('%', ");
        query.push_bind(&name);
        query.push(", '%') OR username LIKE concat('%', ");
        query.push_bind(&name);
        query.push(", '%') ");
    }
    if let Some(order) = UserAccountRow::sorting_to_sql(&sort) {
        query.push(order);
    }
    query.push(" LIMIT ");
    query.push_bind(range.len() as i64);
    query.push(" OFFSET ");
    query.push_bind(range.start as i64);
    query
        .build_query_as::<UserAccountRow>()
        .fetch_all(get_db_pool())
        .await
        .map_err(|e| {
            log!("list_user_accounts error: {e:?}");
            ServerFnError::new(format!("{e:?}"))
        })
}

#[server(endpoint = "/accounts/count", input = server_fn::codec::GetUrl)]
pub async fn accounts_count() -> Result<usize, ServerFnError> {
    //
    use crate::server::get_db_pool;

    let count: i64 = sqlx::query("SELECT COUNT(*) FROM user_accounts")
        .fetch_one(get_db_pool())
        .await
        .map_err(|err| {
            log!("customer_count error: {err:?}");
            ServerFnError::new(format!("{err:?}"))
        })?
        .get(0);

    Ok(count as usize)
}
