use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <section class="flex min-h-screen flex-col relative">
            <div class="flex-1 overflow-y-auto relative">
                <Outlet />
            </div>

            <div class="flex items-center gap-2 bg-stone-500/20 text-stone-500 py-2 px-4 justify-center text-xs sm:text-sm">
                <h1>"© All Rights Reserved"</h1>
                <span>"·"</span>
                <h1>"Alfoi's Inventory"</h1>
            </div>
        </section>
    }
}
