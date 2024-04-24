use chrono::{Local, NaiveDateTime};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::blog_post::BlogPost,
    model::Post,
    repository::{get_post, upsert_post, DeletePost, UpsertPost},
};

#[derive(Params, PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

fn format_dt(dt: NaiveDateTime) -> String {
    dt.format("%Y-%m-%dT%H:%M").to_string()
}

#[component]
pub fn EditPost() -> impl IntoView {
    //
    let params = use_params::<EditPostParams>();

    let post_resource = create_resource(
        move || params.get(),
        |params| async move {
            match params {
                Ok(EditPostParams { post_id: Some(s) }) => get_post(s).await,
                // If no id is provided in the URL, we assume making a new post.
                _ => Ok(Post::new_empty()),
            }
        },
    );

    let upsert_post = create_server_action::<UpsertPost>();
    let delete_post = create_server_action::<DeletePost>();

    view! {
        <Suspense fallback = move || view! { <p>"Loading ..."</p> }>
            <ErrorBoundary fallback = move |_| view! { <p>"Error!"</p> }>
                <div class="flex h-screen">
                    <div class="min-w-[50%] max-h-[90%] text-gray-800 bg-gray-200 p-10 rounded-md">
                        <ActionForm action=upsert_post>
                            <form>
                                <label class="block mb-4">
                                    <span>Date</span>
                                    <input class="mt-1 p-2 w-full" type="datetime-local" id="datetime" name="dt"
                                        on:input=move |ev| {
                                            let dt: String = event_target_value(&ev);
                                            let chrono_dt = NaiveDateTime::parse_from_str(&dt, "%Y-%m-%dT%H:%M");
                                            let utc_dt = match chrono_dt {
                                                Ok(dt) => dt,
                                                _ => Local::now().naive_local()
                                            };
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                    post.dt = utc_dt;
                                                }
                                            });
                                        }
                                        prop:value={move || {
                                            post_resource
                                                .get()
                                                .and_then(|res| res.map(|post| format_dt(post.dt)).ok())
                                        }}
                                    />
                                </label>
                                <label class="block mb-4">
                                    <span>Image URL</span>
                                    <input class="mt-1 p-2 w-full" type="text" id="image_url" name="image_url"
                                        on:input=move |ev| {
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                post.image_url = event_target_value(&ev);
                                                }
                                            });
                                        }
                                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.image_url).ok())}/>
                                </label>
                                <label class="block mb-4">
                                    <span>Title</span>
                                    <input class="mt-1 p-2 w-full" type="text" id="title" name="title"
                                        on:input=move |ev| {
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                    post.title = event_target_value(&ev)
                                                }
                                            })
                                        }
                                        prop:value={ move || post_resource.get().and_then(|res| res.map(|post| post.title).ok())}
                                    />
                                </label>
                                <label class="block mb-4">
                                    <span>Content</span>
                                    <textarea class="mt-1 p-2 w-full" id="text" name="text"
                                        on:input=move |ev| {
                                            post_resource.update(|curr| {
                                                if let Some(Ok(post)) = curr {
                                                    post.content = event_target_value(&ev)
                                                }
                                            })
                                        }
                                        prop:value={ move || post_resource.get().and_then(|res| res.map(|post| post.content).ok())}
                                    />
                                </label>
                                <div class="flex justify-center pb-4">
                                    <input type="submit" value="Submit" class="mx-auto w-1/3 bg-blue-500 hover:bg-blue-700 text-white"/>
                                </div>
                            </form>
                        </ActionForm>
                        <ActionForm action=delete_post>
                            <input type="hidden" name="id"
                                prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.id).ok())}/>
                            <div class="flex justify-center pb-4">
                                <input type="submit" value="Delete Post" class="mx-auto w-1/3 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded cursor-pointer"/>
                            </div>
                        </ActionForm>
                    </div>
                    // The right side preview.
                    <div>
                        { move || post_resource.and_then( |post| view! { <BlogPost post=post.clone() /> } )}
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
