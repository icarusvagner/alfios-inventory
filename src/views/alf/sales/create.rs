use leptos::{ev, html, prelude::*};
use leptos_icons::Icon;

use crate::{
    config_provider::ContextConfigProvider,
    views::alf::{comp_view::ComponentView, Item, TaxGroup},
};

#[component]
pub fn SalesCreateView() -> impl IntoView {
    let context = ContextConfigProvider::expect_context();
    let title = RwSignal::new(String::new());
    let title_ref = NodeRef::<html::Input>::new();
    let list_items = RwSignal::new([].to_vec());

    request_animation_frame(move || {
        if let Some(title_input) = title_ref.get_untracked() {
            let _ = title_input.focus();
        }
    });

    let add_item = move || {
        let focus_ref = NodeRef::<html::Input>::new();

        let new_item = Item {
            item_name: RwSignal::new(String::new()),
            qty: RwSignal::new(0),
            price: RwSignal::new(0.0),
            tax_group: RwSignal::new(TaxGroup::get_tax(TaxGroup::Food)),
            focus_ref,
        };
        list_items.update(|val| val.push(new_item));

        request_animation_frame(move || {
            if let Some(input) = focus_ref.get_untracked() {
                let _ = input.focus();
            }
        });
    };

    let compute_total = move |qty: f32, price: f32, tax: f32| {
        let subtotal = qty * price;
        subtotal + (subtotal * tax)
    };

    let on_keypress_title = move |e: ev::KeyboardEvent| {
        title.set(event_target_value(&e));
        if e.key().eq(&"Enter") && !title.get().is_empty() {
            add_item();
        }
    };

    view! {
        <section class="h-full flex flex-col gap-5 p-3">
            <div class="flex gap-4">
                <button
                    on:click=move |_| context.comp_view.set(ComponentView::Display)
                    class="cursor-pointer bg-blue-500 text-stone-100 flex items-center justify-center p-1.5 rounded hover:scale-95 duration-300 ease-in-out"
                >
                    <Icon icon=icondata::BsArrowLeftShort attr:class="h-6 w-6" />
                </button>
                <input
                    node_ref=title_ref
                    on:keypress=on_keypress_title
                    prop:value=move || title.get()
                    class="flex-1 px-3 outline-none border border-stone-400 py-1.5"
                    placeholder="Sale name"
                />
                <button class="cursor-pointer rounded bg-red-500 text-stone-100 text-base px-3 hover:scale-95 duration-300 ease-in-out">
                    "Cancel"
                </button>
                <button class="cursor-pointer rounded bg-green-500 text-stone-100 text-base px-3 hover:scale-95 duration-300 ease-in-out">
                    "Save"
                </button>
            </div>
            <div class="overflow-x-auto bg-stone-200 space-y-4 p-3">
                <button
                    on:click=move |_| {
                        add_item();
                    }
                    class="flex items-center cursor-pointer gap-2 bg-blue-500 p-2 text-stone-50 text-sm rounded hover:scale-95 duration-300 ease-in-out"
                >
                    <Icon icon=icondata::AiPlusOutlined attr:class="h-4 w-4" />
                    " Add Item"
                </button>

                <div class="min-w-full grid grid-cols-9 gap-2">
                    <div class="col-span-4">"Item name"</div>
                    <div class="col-span-1">"Qty"</div>
                    <div class="col-span-1">"Price"</div>
                    <div class="col-span-1">"Tax Group"</div>
                    <div class="col-span-2 text-right">"Total"</div>

                    {move || {
                        list_items
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(i, item)| {
                                view! {
                                    <div class="col-span-4">
                                        <input
                                            node_ref=item.focus_ref
                                            class="bg-white w-full px-2 py-1 outline-none"
                                            placeholder="Item name"
                                            on:input=move |e| item.item_name.set(event_target_value(&e))
                                            prop:value=move || item.item_name.get()
                                        />
                                    </div>
                                    <div class="col-span-1">
                                        <input
                                            type="number"
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
                                    </div>
                                    <div class="col-span-1">
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
                                    </div>
                                    <div class="col-span-1">
                                        <select
                                            on:change=move |ev| {
                                                item.tax_group
                                                    .set(event_target_value(&ev).parse::<f32>().unwrap());
                                            }
                                            class="w-full h-full rounded outline-none bg-white sm:text-sm"
                                        >
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
                                    </div>
                                    <div class="col-span-2 flex items-center justify-end gap-2">
                                        <span>
                                            {move || {
                                                format!(
                                                    "${:.2}",
                                                    compute_total(
                                                        item.qty.get() as f32,
                                                        item.price.get(),
                                                        item.tax_group.get(),
                                                    ),
                                                )
                                            }}
                                        </span>
                                        <button
                                            on:click=move |_| {
                                                list_items
                                                    .update(|val| {
                                                        val.remove(i);
                                                    })
                                            }
                                            class="rounded bg-red-500 p-0.5 text-stone-50 hover:scale-95 duration-300 ease-in-out cursor-pointer"
                                        >
                                            <Icon icon=icondata::MdiWindowClose attr:class="h-5 w-5" />
                                        </button>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>
            </div>
        </section>
    }
}
