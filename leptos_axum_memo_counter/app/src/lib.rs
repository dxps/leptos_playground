#![allow(non_snake_case)]

use leptos::{logging::log, prelude::*, reactive::spawn_local, server, server_fn::codec::PutUrl};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_increase = move |_| *count.write() += 1;

    let on_server = move |_| {
        spawn_local(async move {
            if let Err(err) = set_server_increment(count.get_untracked()).await {
                log!("Got error from server when setting the couter: {}", err);
            };
        });
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_increase>"Increase"</button>
        <h3>{count}</h3>
        <hr/>
        <button on:click=on_server>"Send it to the server"</button>
    }
}

#[server(CounterUpdate, endpoint = "/counter/set")]
pub async fn set_server_increment(counter: i32) -> Result<String, ServerFnError> {
    println!("[set_server_increment] counter: {}", counter);

    Ok("Ack".into())
}
