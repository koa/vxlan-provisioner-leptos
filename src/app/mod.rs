pub mod auth_context;
mod components;
mod pages;

use crate::app::{
    auth_context::provide_auth,
    components::{menu_entry::RouterMainMenuEntry, sidebar::SidebarMenu},
    pages::device::DevicePage,
};
use leptos::{
    component,
    hydration::{AutoReload, HydrationScripts},
    logging::log,
    prelude::{
        expect_context, server, server_fn, ClassAttribute, CollectView, ElementChild, Get,
        GlobalAttributes, LeptosOptions, LocalResource, OnAttribute, RwSignal, ServerFnError,
        Write,
    },
    task::spawn_local,
    view, IntoView,
};
#[cfg(feature = "hydrate")]
use leptos::{
    logging::error,
    prelude::{use_context, Effect, Set},
};
use leptos_meta::{provide_meta_context, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment, WildcardSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        // "dark" Klasse für Tailwind V4/V3
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta content="width=device-width, initial-scale=1.0" name="viewport" />
                <link
                    href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300;400;500;600;700&amp;family=Inter:wght@300;400;500;600;700&amp;display=swap"
                    rel="stylesheet"
                />
                <link
                    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&amp;display=swap"
                    rel="stylesheet"
                />
                <link id="leptos" href="/pkg/vxlan-provisioner-leptos.css" rel="stylesheet" />
                <script src="https://accounts.google.com/gsi/client" async defer></script>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone() />
                <MetaTags />
            </head>
            // Hier direkt die Klassen für das Sentinel-Design setzen
            <body class="bg-background text-on-surface">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_auth();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Title text="Welcome to Leptos" />
        // content for this welcome page
        <Router>
            <SidebarMenu />
            <main class="ml-64 p-8 min-h-screen bg-background text-on-surface">
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("/device/:device_id") view=DevicePage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    LocalResource::new(move || async {});
    let server_lick = move |_| {
        spawn_local(async {
            //login_settings().await;
        })
    };
    let dev_resource = LocalResource::new(move || async {
        log!("refresh");
        Vec::<(u32, String)>::new()
        /*list_devices().await.unwrap_or_else(|e| {
            error!("Error from server: {:?}", e);
            Vec::new()
        })*/
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <aside>
            <nav>
                <ul>
                    {move || {
                        dev_resource
                            .get()
                            .iter()
                            .flatten()
                            .map(|device| {
                                view! {
                                    <li>
                                        <a href=format!(
                                            "/device/{}",
                                            device.0,
                                        )>{device.1.clone()}</a>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
            </nav>
        </aside>
        <nav>
            <ul>
                <li>
                    <button on:click=on_click>"Click Me: " {count}</button>
                </li>
                <li>
                    <button on:click=server_lick>"Server Click"</button>
                </li>
            </ul>
        </nav>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

#[server]
async fn list_devices(token: Box<str>) -> Result<Vec<(u32, String)>, ServerFnError> {
    let devices = crate::server::list_devices(token.as_ref()).await?;
    Ok(devices)
}
