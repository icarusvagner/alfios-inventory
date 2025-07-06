use leptos::prelude::*;
use leptos_icons::Icon;

use crate::{config_provider::ContextConfigProvider, views::alf::comp_view::ComponentView};

#[derive(Debug, Default, Clone)]
struct TempData {
    field: Vec<(String, f32)>,
}

#[component]
pub fn SalesDisplayView() -> impl IntoView {
    let context = ContextConfigProvider::expect_context();
    let sales = RwSignal::new(TempData::default());

    let on_click = move |_| context.comp_view.set(ComponentView::Create);

    view! {
        <div class="h-full space-y-5 p-3">
            {move || {
                if !sales.get().field.is_empty() {
                    view! {
                        <div class="flex items-center justify-end">
                            <button class="px-5 py-1.5 rounded hover:scale-95 duration-200 ease-in-out shadow-2xl bg-green-500 text-stone-50 cursor-pointer">
                                "New Sale"
                            </button>
                        </div>
                        <hr class="text-stone-500/30" />
                    }
                        .into_any()
                } else {
                    view! {
                        <button
                            on:click=on_click
                            class="bg-blue-500 flex items-center gap-2 outline-none px-5 py-2 absolute top-1/2 -translate-y-1/2 -translate-x-1/2 left-1/2 text-stone-100 cursor-pointer duration-200 ease-in-out hover:scale-95"
                        >
                            "Create your first sale"
                            <Icon icon=icondata::BsArrowRight attr:class="h-4 w-4" />
                        </button>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
