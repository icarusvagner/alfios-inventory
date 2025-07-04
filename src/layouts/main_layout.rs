use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <section class="flex min-h-screen flex-col relative">
            <Outlet />
        </section>
    }
}
