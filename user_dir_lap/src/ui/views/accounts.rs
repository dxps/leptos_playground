use crate::{
    domain::model::Id,
    ui::logic::{UserAccountQuery, UserAccountRow, accounts_count, list_user_accounts},
};
use leptos::prelude::*;
use leptos_struct_table::*;
use std::collections::VecDeque;
use std::ops::Range;

#[derive(TableRow, Clone)]
#[table(sortable, classes_provider = "TailwindClassesPreset")]
struct UserAccount {
    pub id: Id,
    pub name: String,
    pub username: String,
    pub email: String,
}

#[component]
pub fn UserAccountsView() -> impl IntoView {
    //
    let scroll_container = NodeRef::new();
    let rows = UserAccountsDataProvider::default();

    view! {
        <div class="mt-10">
            <h2>"User Accounts"</h2>
        </div>
        <div node_ref=scroll_container class="shadow-lg rounded-lg overflow-clip m-2 float-left min-w-10/12">
            <table class="text-sm text-left text-gray-500 mb-[-1px] w-full">
                <TableContent
                    rows
                    scroll_container
                />
            </table>
        </div>
    }
}

///////////////////
// Data Provider //
///////////////////

#[derive(Default)]
pub struct UserAccountsDataProvider {
    sort: VecDeque<(usize, ColumnSort)>,
    pub name: RwSignal<String>,
}

impl TableDataProvider<UserAccountRow> for UserAccountsDataProvider {
    async fn get_rows(
        &self,
        range: Range<usize>,
    ) -> Result<(Vec<UserAccountRow>, Range<usize>), String> {
        list_user_accounts(UserAccountQuery {
            name: self.name.get_untracked().trim().to_string(),
            sort: self.sort.clone(),
            range: range.clone(),
        })
        .await
        .map(|rows| {
            let len = rows.len();
            (rows, range.start..range.start + len)
        })
        .map_err(|e| format!("{e:?}"))
    }

    async fn row_count(&self) -> Option<usize> {
        accounts_count().await.ok()
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sort = sorting.clone();
    }

    fn track(&self) {
        self.name.track();
    }
}
