use leptos::logging::{error, log};
use leptos::{logging, prelude::*, task::spawn_local};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/vxlan-provisioner-leptos.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />
        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
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
    let server_lick = move |_| {
        spawn_local(async {
            test_server().await;
        })
    };
    let dev_resource = LocalResource::new(move || async {
        log!("refresh");
        match list_devices().await {
            Ok(devices) => devices,
            Err(e) => {
                error!("Error from server: {:?}", e);
                Vec::new()
            }
        }
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
async fn test_server() -> Result<(), ServerFnError> {
    logging::log!("test_server");
    Ok(())
}

#[server]
async fn list_devices() -> Result<Vec<(u32, String)>, ServerFnError> {
    let devices = crate::server::list_devices().await?;
    Ok(devices)
}
