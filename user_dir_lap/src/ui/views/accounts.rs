use crate::domain::model::Id;
use leptos::prelude::*;
use leptos_struct_table::*;
use std::collections::VecDeque;

#[derive(TableRow, Clone)]
#[table(sortable, classes_provider = "TailwindClassesPreset")]
struct UserAccount {
    pub id: Id,
    pub name: String,
    pub username: String,
    pub email: String,
}

#[component]
pub fn UserAccounts() -> impl IntoView {
    //
    let rows = UserAccountsDataProvider::default();
    let pagination_controller = PaginationController::default();

    view! {
        <div>
            <h2>"User Accounts"</h2>
        </div>
        <div class="shadow-lg rounded-lg overflow-clip m-10 float-left">
            <table class="text-sm text-left text-gray-500 mb-[-1px]">
                <TableContent
                    rows
                    scroll_container="html"
                    sorting_mode=SortingMode::SingleColumn
                    display_strategy=DisplayStrategy::Pagination {
                        controller: pagination_controller,
                        row_count: 5,
                    }
                />
            </table>
        </div>
        <Paginator pagination_controller />
    }
}

#[component]
pub fn Paginator(pagination_controller: PaginationController) -> impl IntoView {
    let current_page = pagination_controller.current_page;
    let page_count = pagination_controller.page_count();

    let page_range = move || {
        let mut start = current_page.get().saturating_sub(2);
        let mut end = start + 5;

        if let Some(row_count) = page_count.get() {
            if end > row_count {
                end = row_count;
                start = end.saturating_sub(5);
            }
        }

        start..end
    };

    view! {
        <nav aria-label="Page navigation" class="m-10 flex justify-end">
            <ul class="inline-flex -space-x-px text-sm">
                <li>
                    <a
                        href="#"
                        class="flex items-center justify-center px-3 h-8 ms-0 leading-tight text-gray-500 bg-white border border-e-0 border-gray-300 rounded-s-lg hover:bg-gray-100 hover:text-gray-700"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.current_page.set(0);
                        }
                    >
                        First
                    </a>
                </li>
                <li>
                    <a
                        href="#"
                        class="flex items-center justify-center px-3 h-8 ms-0 leading-tight text-gray-500 bg-white border border-e-0 border-gray-300 hover:bg-gray-100 hover:text-gray-700"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.previous();
                        }
                    >
                        Previous
                    </a>
                </li>

                <For each=page_range key=|page| *page let:page>
                    <PageLink page pagination_controller />
                </For>

                <li>
                    <a
                        href="#"
                        class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.next();
                        }
                    >
                        Next
                    </a>
                </li>

                <li>
                    <a
                        href="#"
                        class="flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 rounded-e-lg hover:bg-gray-100 hover:text-gray-700"
                        on:click=move |evt| {
                            evt.prevent_default();
                            evt.stop_propagation();
                            pagination_controller.current_page.set(page_count.get().unwrap_or(1)-1);
                        }
                    >
                        Last
                    </a>
                </li>

            </ul>
        </nav>
    }
}

#[component]
pub fn PageLink(page: usize, pagination_controller: PaginationController) -> impl IntoView {
    let is_selected = move || pagination_controller.current_page.get() == page;

    let class = move || {
        if is_selected() {
            "flex items-center justify-center px-3 h-8 text-blue-600 border border-gray-300 bg-blue-50 hover:bg-blue-100 hover:text-blue-700"
        } else {
            "flex items-center justify-center px-3 h-8 leading-tight text-gray-500 bg-white border border-gray-300 hover:bg-gray-100 hover:text-gray-700"
        }
    };

    view! {
        <li>
            <a
                href="#"
                class=class
                on:click=move |evt| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    pagination_controller.current_page.set(page);
                }
            >
                {page + 1}
            </a>
        </li>
    }
}

/////////////////////////////////////////////////////////////////////
// Data Provider
/////////////////////////////////////////////////////////////////////

pub struct UserAccountsDataProvider {
    sorting: VecDeque<(usize, ColumnSort)>,
}

impl Default for UserAccountsDataProvider {
    fn default() -> Self {
        Self {
            sorting: VecDeque::new(),
        }
    }
}

impl PaginatedTableDataProvider<UserAccount> for UserAccountsDataProvider {
    const PAGE_ROW_COUNT: usize = 50;

    async fn get_page(&self, page_index: usize) -> Result<Vec<UserAccount>, String> {
        // Just for testing.
        let data: Vec<UserAccount> = vec![
            UserAccount {
                id: Id::new_from("1".to_string()),
                email: "someone@email.com".into(),
                name: "John Doe".into(),
                username: "johndoe".into(),
            },
            UserAccount {
                id: Id::new_from("2".to_string()),
                email: "someother@email.com".into(),
                name: "Jane Doe".into(),
                username: "janedoe".into(),
            },
            UserAccount {
                id: Id::new_from("3".to_string()),
                email: "someother@email.com".into(),
                name: "Jane Doe".into(),
                username: "janedoe".into(),
            },
            UserAccount {
                id: Id::new_from("4".to_string()),
                email: "someother@email.com".into(),
                name: "Jane Doe".into(),
                username: "janedoe".into(),
            },
            UserAccount {
                id: Id::new_from("5".to_string()),
                email: "someother@email.com".into(),
                name: "Jane Doe".into(),
                username: "janedoe".into(),
            },
            UserAccount {
                id: Id::new_from("6".to_string()),
                email: "someother@email.com".into(),
                name: "Jane Doe".into(),
                username: "janedoe".into(),
            },
            UserAccount {
                id: Id::new_from("7".to_string()),
                email: "someother@email.com".into(),
                name: "Jane Doe".into(),
                username: "janedoe".into(),
            },
        ];
        leptos::logging::log!("[PaginatedTableDataProvider] get_page: {}", page_index);
        if page_index == 0 {
            Ok(data)
        } else {
            Ok(Vec::new())
        }
    }

    async fn row_count(&self) -> Option<usize> {
        Some(7)
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sorting = sorting.clone();
    }
}
