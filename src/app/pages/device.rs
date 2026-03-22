use leptos::{
    component,
    prelude::{ElementChild, Read},
    view, IntoView, Params,
};
use leptos::prelude::*;
use leptos::prelude::Get;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq, Clone, Copy)]
struct DeviceParams {
    device_id: u32,
}

#[component]
pub fn DevicePage() -> impl IntoView {
    let params = use_params::<DeviceParams>();
    view! {
        <h1>
            Device: {move || { params.get().ok().map(|p| p.device_id).unwrap_or_default() }}
        </h1>
    }
}
