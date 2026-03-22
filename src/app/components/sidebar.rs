use crate::app::list_devices;
use crate::app::RouterMainMenuEntry;
use leptos::leptos_dom::{error, log};
use leptos::prelude::ClassAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::Get;
use leptos::prelude::{ElementChild, LocalResource};
use leptos::{component, view, IntoView};

#[component]
pub fn SidebarMenu() -> impl IntoView {
    let known_devices = LocalResource::new(move || async {
        //log!("refresh");
        let mut devices = list_devices().await.unwrap_or_else(|e| {
            error!("Error from server: {:?}", e);
            Vec::new()
        });
        devices.sort_by(|(_,a),(_,b)| a.cmp(b));
        devices
    });
    view! {
        <aside class="fixed left-0 top-0 h-screen w-64 bg-slate-900 dark:bg-[#0b1326] z-40 flex flex-col border-r border-slate-800/50">
            <div class="p-6 flex-shrink-0">
                <h1 class="text-2xl font-black text-[#7bd0ff] tracking-tighter font-space-grotesk">
                    Sentinel Lens
                </h1>
                <p class="text-[10px] uppercase tracking-[0.2em] text-slate-500 font-bold mt-1">
                    Network Operations Center
                </p>
            </div>
            <div class="flex-1 overflow-y-auto sidebar-scroll">
                <nav class="px-2 pb-6 space-y-1">
                    <div class="px-4 py-2 text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-2 sticky top-0 bg-[#0b1326] z-10">
                        Device Inventory
                    </div>
                    {move || {
                        known_devices
                            .get()
                            .iter()
                            .flatten()
                            .map(|(id, name)| {
                                view! { <RouterMainMenuEntry icon="router" text=name id=*id /> }
                            })
                            .collect_view()
                    }}
                </nav>
            </div>
            <div class="flex-shrink-0 bg-[#0b1326] border-t border-slate-800/50 p-2 space-y-1">
                <a
                    class="flex items-center gap-3 px-4 py-3 text-slate-400 hover:text-[#dae2fd] hover:bg-[#2d3449] transition-all duration-200"
                    href="#"
                >
                    <span class="material-symbols-outlined" data-icon="notification_important">
                        notification_important
                    </span>
                    <span class="font-space-grotesk tracking-tight">Alerts</span>
                </a>
                <a
                    class="flex items-center gap-3 px-4 py-3 text-slate-400 hover:text-[#dae2fd] hover:bg-[#2d3449] transition-all duration-200"
                    href="#"
                >
                    <span class="material-symbols-outlined" data-icon="settings">
                        settings
                    </span>
                    <span class="font-space-grotesk tracking-tight">Settings</span>
                </a>
            </div>
        </aside>
    }
}
