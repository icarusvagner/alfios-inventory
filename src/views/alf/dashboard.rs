use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Debug, Default, Clone)]
struct TempData {
    field: Vec<(String, f32)>,
}

#[component]
pub fn DashboardALF() -> impl IntoView {
    let sales = RwSignal::new(TempData::default());

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
                        <A
                            href="/sales/new"
                            attr:class="bg-blue-500 px-5 py-2 absolute top-1/2 -translate-y-1/2 -translate-x-1/2 left-1/2 text-stone-100 cursor-pointer hover:scale-95 duration-200 ease-in-out"
                        >
                            "Create your first sale"
                        </A>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
