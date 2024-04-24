use leptos::*;

use crate::model::Post;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    //
    let dt = format!("{}", post.dt.format("%B %e, %Y %I:%M%P"));
    view! {
        <div class="block p-10">
            <div class="text-xl">{dt}</div>
            <img src={&post.image_url} alt="Post thumbnail" class="w-96 h-32 rounded-lg object-cover my-10" />
            <div class="text-2xl pb-4">{&post.title}</div>
            <div>{&post.content}</div>
        </div>
    }
}
