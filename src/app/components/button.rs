use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};

#[component]
pub fn PrimaryButton(#[prop(into)] title: String, #[prop(into)] icon: String) -> impl IntoView {
    view! {
        // Die gesamte Tailwind-Logik aus dem Stitch-Projekt wird hier einmalig gekapselt
        <button class="bg-primary text-on-primary hover:bg-primary-fixed-dim transition-all px-6 py-3 rounded font-bold text-xs uppercase tracking-widest flex items-center gap-3 group whitespace-nowrap">
            <span class="material-symbols-outlined text-lg group-hover:rotate-12 transition-transform">
                {icon}
            </span>
            {title}
        </button>
    }
}
