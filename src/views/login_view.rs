use leptos::prelude::*;

#[component]
pub fn LoginView() -> impl IntoView {
    view! {
        <section class="min-h-screen flex items-center justify-center">
            <div class="min-w-xl rounded shadow-2xl p-12">
                <h1 class="text-4xl font-black text-center tracking-wide mb-10 font-geist">
                    "Alfio's "<span class="text-blue-500 font-poppins">"Inventory"</span>
                </h1>

                <div class="flex flex-col gap-1 w-full mb-8">
                    <label for="username" class="text-stone-400">
                        "Username *"
                    </label>
                    <input
                        id="username"
                        required
                        type="text"
                        class="border border-stone-400 rounded py-2 px-3 outline-none"
                        placeholder="Enter your username"
                    />
                </div>

                <div class="flex flex-col gap-1 w-full">
                    <label for="password" class="text-stone-400">
                        "Password *"
                    </label>
                    <input
                        id="password"
                        required
                        type="password"
                        class="border border-stone-400 rounded py-2 px-3 outline-none"
                        placeholder="*********"
                    />
                </div>

                <button
                    type="submit"
                    class="mt-10 w-full text-center text-xl font-bold tracking-widest uppercase bg-sky-600 py-2 rounded text-stone-50"
                >
                    "Submit"
                </button>
            </div>
        </section>
    }
}
