use leptos::{ev, prelude::*};
use leptos_icons::Icon;

use crate::{config_provider::ContextConfigProvider, views::alf::comp_view::ComponentView};

#[derive(Clone)]
struct Item {
    item_name: RwSignal<String>,
    qty: RwSignal<u32>,
    price: RwSignal<f32>,
    tax_group: RwSignal<TaxGroup>,
}

#[derive(Debug, Clone, Copy)]
enum TaxGroup {
    Food,
    Alcohol,
    Nontaxable,
    Other,
}

impl TaxGroup {
    pub const ALL: [TaxGroup; 4] = [
        TaxGroup::Food,
        TaxGroup::Alcohol,
        TaxGroup::Nontaxable,
        TaxGroup::Other,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            TaxGroup::Food => "Food (8%)",
            TaxGroup::Alcohol => "Alcohol (10%)",
            TaxGroup::Nontaxable => "Non-taxable",
            TaxGroup::Other => "Other (8%)",
        }
    }

    pub fn get_tax(tax: TaxGroup) -> f32 {
        match tax {
            TaxGroup::Food => 0.08,
            TaxGroup::Alcohol => 0.10,
            TaxGroup::Nontaxable => 0.0,
            TaxGroup::Other => 0.08,
        }
    }
}

#[component]
pub fn SalesCreateView() -> impl IntoView {
    let context = ContextConfigProvider::expect_context();
    let title = RwSignal::new(String::new());
    let list_items = RwSignal::new([].to_vec());

    let add_item = move || {
        let new_item = Item {
            item_name: RwSignal::new(String::new()),
            qty: RwSignal::new(0),
            price: RwSignal::new(0.0),
            tax_group: RwSignal::new(TaxGroup::Food),
        };
        list_items.update(|val| val.push(new_item));
    };

    let on_keypress_title = move |e: ev::KeyboardEvent| {
        if e.key() == "Enter" && !title.get().trim().is_empty() {
            add_item();
            title.set(event_target_value(&e));
        }
    };

    view! {
        <section class="h-full flex flex-col gap-5 p-3">
            <div class="flex gap-4">
                <button
                    on:click=move |_| context.comp_view.set(ComponentView::Display)
                    class="cursor-pointer bg-blue-500 text-stone-100 flex items-center justify-center p-1.5 rounded hover:scale-95 duration-300 ease-in-out"
                >
                    <Icon icon=icondata::BsArrowLeftShort attr:class="h-10 w-10" />
                </button>
                <input
                    on:keypress=on_keypress_title
                    prop:value=move || title.get()
                    class="flex-1 px-3 outline-none border border-stone-400 py-1.5"
                    placeholder="Sale name"
                />
                <button class="cursor-pointer rounded bg-red-500 text-stone-100 text-base px-3">
                    "Cancel"
                </button>
                <button class="cursor-pointer rounded bg-green-500 text-stone-100 text-base px-3">
                    "Save"
                </button>
            </div>
            <div class="overflow-x-auto bg-stone-200 space-y-4 p-3">
                <button
                    on:click=move |_| {
                        add_item();
                    }
                    class="flex items-center gap-2 bg-blue-500 p-2 text-stone-50 text-sm rounded"
                >
                    <Icon icon=icondata::AiPlusOutlined attr:class="h-4 w-4" />
                    " Add Item"
                </button>
                <table class="min-w-full divide-y-2 divide-gray-200">
                    <thead class="ltr:text-left rtl:text-right">
                        <tr class="*:font-medium *:text-gray-900">
                            <th class="px-3 py-2 whitespace-nowrap">"Item Name"</th>
                            <th class="px-3 py-2 whitespace-nowrap w-24">"Qty"</th>
                            <th class="px-3 py-2 whitespace-nowrap w-24">"Price"</th>
                            <th class="px-3 py-2 whitespace-nowrap w-24">"Tax Group"</th>
                            <th class="px-3 py-2 whitespace-nowrap w-52 text-right">"Total"</th>
                        </tr>
                    </thead>

                    <tbody class="">
                        {move || {
                            list_items
                                .get()
                                .into_iter()
                                .enumerate()
                                .map(|(i, item)| {
                                    view! {
                                        <tr class="*:text-gray-900 *:first:font-medium mb-3">
                                            <td class="pr-3">
                                                <input
                                                    class="bg-white w-full px-2 py-1 outline-none"
                                                    placeholder="Item name"
                                                    on:input=move |e| item.item_name.set(event_target_value(&e))
                                                    prop:value=move || item.item_name.get()
                                                />
                                            </td>
                                            <td class="pr-3">
                                                <input
                                                    class="bg-white w-full px-2 py-1 outline-none"
                                                    placeholder="Quantity"
                                                    min="0"
                                                    max="999"
                                                    on:input=move |e| {
                                                        if let Ok(v) = event_target_value(&e).parse() {
                                                            item.qty.set(v);
                                                        }
                                                    }
                                                    prop:value=move || item.qty.get()
                                                />
                                            </td>
                                            <td class="pr-3">
                                                <input
                                                    type="number"
                                                    min="0"
                                                    max="999"
                                                    class="bg-white w-full px-2 py-1 outline-none"
                                                    placeholder="Price"
                                                    on:input=move |e| {
                                                        if let Ok(v) = event_target_value(&e).parse() {
                                                            item.price.set(v);
                                                        }
                                                    }
                                                    prop:value=move || item.price.get()
                                                />
                                            </td>
                                            <td class="pr-3">
                                                <select class="w-full rounded outline-none bg-white sm:text-sm">
                                                    {move || {
                                                        TaxGroup::ALL[..]
                                                            .iter()
                                                            .map(|val| {
                                                                view! {
                                                                    <option value=TaxGroup::get_tax(
                                                                        *val,
                                                                    )>{val.as_str()}</option>
                                                                }
                                                                    .into_any()
                                                            })
                                                            .collect_view()
                                                    }}
                                                </select>
                                            </td>
                                            <td class="flex items-center justify-end gap-1">
                                                <span>
                                                    {move || {
                                                        format!("${:.2}", item.price.get() * item.qty.get() as f32)
                                                    }}
                                                </span>
                                                <button
                                                    on:click=move |_| {
                                                        list_items
                                                            .update(|val| {
                                                                val.remove(i);
                                                            })
                                                    }
                                                    class="rounded bg-red-500"
                                                >
                                                    <Icon
                                                        icon=icondata::MdiWindowClose
                                                        attr:class="h-5 w-5 text-stone-50"
                                                    />
                                                </button>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect_view()
                        }}
                    </tbody>
                </table>
            </div>
        </section>
    }
}
