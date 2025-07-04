use leptos::prelude::*;

use crate::config_provider::ContextConfigProvider;

#[component]
pub fn DashboardALF() -> impl IntoView {
    let context = ContextConfigProvider::expect_context().comp_view;

    view! { {move || context.get().to_view()} }
}
