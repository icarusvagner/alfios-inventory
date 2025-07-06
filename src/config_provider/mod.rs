mod login_provider;

use leptos::{context::Provider, prelude::*};

use crate::views::alf::{comp_view::ComponentView, SalesDataWrapper};

#[derive(Clone)]
pub struct ContextConfigProvider {
    pub comp_view: RwSignal<ComponentView>,
    pub sales_data: SalesDataWrapper,
    pub overlay: RwSignal<bool>,
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
    let sales_data = SalesDataWrapper::default();
    let overlay = RwSignal::new(false);

    let config_provider = ContextConfigProvider {
        comp_view,
        sales_data,
        overlay,
    };

    view! { <Provider value=config_provider>{children()}</Provider> }
}
