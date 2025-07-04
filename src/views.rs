mod login_view;

pub use login_view::*;

pub mod alf;

use leptos::prelude::*;

#[component]
pub fn NotFoundView() -> impl IntoView {
    view! {
        <section class="flex min-h-screen flex-col items-center justify-center">
            <h1 class="text-9xl font-black tracking-wider">"404"</h1>
            <h4 class="my-10 text-xl uppercase">"page not found"</h4>
            <a
                href="/"
                class="bg-stone-500/40 px-5 py-2 rounded text-stone-800 hover:scale-95 duration-200 ease-in-out"
            >
                "Back home"
            </a>
        </section>
    }
}
