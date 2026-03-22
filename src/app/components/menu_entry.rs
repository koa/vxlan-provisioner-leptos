use leptos::prelude::AddAnyAttr;
use leptos::{
    component,
    prelude::{ClassAttribute, Component, CustomAttribute, ElementChild},
    view, IntoView,
};
use leptos_router::components::A;
#[component]
pub fn RouterMainMenuEntry(
    icon: &'static str,
    #[prop(into)] text: String,
    #[prop(into)] id: u32,
) -> impl IntoView {
    let path = format!("/device/{}", id);
    view! { <NavItem icon=icon label=text href=path /> }
}
#[component]
pub fn NavItem(
    #[prop(into)] href: String,
    icon: &'static str,
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <A href=href attr:class="nav-item">
            <span class="material-symbols-outlined" data-icon=icon>
                {icon}
            </span>
            <span class="font-space-grotesk tracking-tight">{label}</span>
        </A>
    }
}
