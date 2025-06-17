use crate::{
    server_fns_todo::{AddTodo, DeleteTodo, get_todos},
    ui::comps::ErrorTemplate,
};
use leptos::{either::Either, prelude::*};

#[component]
pub fn Todos() -> impl IntoView {
    let add_todo = ServerMultiAction::<AddTodo>::new();
    let submissions = add_todo.submissions();
    let delete_todo = ServerAction::<DeleteTodo>::new();

    // The list of todos is loaded from the server as a reaction to changes.
    let todos = Resource::new(
        move || {
            (
                delete_todo.version().get(),
                add_todo.version().get(),
                delete_todo.version().get(),
            )
        },
        move |_| get_todos(),
    );

    let existing_todos = move || {
        Suspend::new(async move {
            todos.await.map(|todos| {
                if todos.is_empty() {
                    Either::Left(view! { <p>"No tasks were found."</p> })
                } else {
                    Either::Right(
                        todos
                            .iter()
                            .map(move |todo| {
                                let id = todo.id;
                                view! {
                                    <li>
                                        {todo.title.clone()}
                                        <ActionForm action=delete_todo>
                                            <input type="hidden" name="id" value=id/>
                                            <input type="submit" value="X"/>
                                        </ActionForm>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            })
        })
    };

    view! {
        <MultiActionForm action=add_todo>
            <label>"Add a Todo" <input type="text" name="title"/></label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
        <div>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| view! { <ErrorTemplate errors/> }>
                    <ul>
                        {existing_todos}
                        {move || {
                            submissions
                                .get()
                                .into_iter()
                                .filter(|submission| submission.pending().get())
                                .map(|submission| {
                                    view! {
                                        <li class="pending">
                                            {move || submission.input().get().map(|data| data.title)}
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}

                    </ul>
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
