use crate::model::device::{DeviceListEntry, DeviceRole};
use leptos::{
    component,
    prelude::{AddAnyAttr, ClassAttribute, CustomAttribute, ElementChild},
    view, IntoView,
};
use leptos_router::components::A;

#[component]
pub fn RouterMainMenuEntry(device: DeviceListEntry) -> impl IntoView {
    let id = device.id;
    let icon = match device.role {
        None => "question_mark",
        Some(DeviceRole::Router) => "network_node",
        Some(DeviceRole::Switch) => "lan",
        Some(DeviceRole::AccessPoint) => "sensors",
    };
    //let icon="router";
    let text = device.name;
    let type_name = device.device_type_name;
    let path = format!("/device/{}", id);
    view! { <NavItem icon=icon label=text href=path subtitle=type_name /> }
}
#[component]
pub fn NavItem(
    #[prop(into)] href: String,
    icon: &'static str,
    #[prop(into)] label: String,
    #[prop(optional)] subtitle: Option<String>,
) -> impl IntoView {
    let subtitle_content = subtitle.map(|subtitle| view! {
        <span class="text-[9px] font-sans text-slate-400/80 uppercase tracking-wider font-medium">
            {subtitle}
        </span>
    });
    view! {
        <A href=href attr:class="nav-item">
            <span class="material-symbols-outlined" data-icon=icon>
                {icon}
            </span>
            <span class="font-space-grotesk tracking-tight">
                <div class="flex flex-col">
                    <span class="font-space-grotesk tracking-tight">{label}</span>
                    {subtitle_content}
                </div>
            </span>
        </A>
    }
}
