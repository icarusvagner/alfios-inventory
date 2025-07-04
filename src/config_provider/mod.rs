use leptos::{context::Provider, prelude::*};

use crate::views::alf::comp_view::ComponentView;

#[derive(Clone)]
pub struct ContextConfigProvider {
    pub comp_view: RwSignal<ComponentView>,
}

impl ContextConfigProvider {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn to_set(&self, comp: ComponentView) {
        self.comp_view.set(comp);
    }
}

#[component]
pub fn ConfigProvider(children: Children) -> impl IntoView {
    let comp_view = RwSignal::new(ComponentView::Display);
    let config_provider = ContextConfigProvider { comp_view };

    view! { <Provider value=config_provider>{children()}</Provider> }
}
