use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::config_provider::ContextConfigProvider;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <section class="flex min-h-screen flex-col relative">
            <Outlet />
        </section>

        <LoadingComponent />
    }
}

#[component]
pub fn LoadingComponent() -> impl IntoView {
    let context = ContextConfigProvider::expect_context();

    view! {
        <Show when=context.overlay fallback=|| ()>
            <section class="absolute min-h-screen w-ful inset-0 z-[9999] flex items-center justify-center bg-stone-200">
                <svg
                    class="h-42 w-42"
                    xmlns="http://www.w3.org/2000/svg"
                    width="100"
                    height="100"
                    viewBox="0 0 100 100"
                    overflow="visible"
                    fill="#1248fc"
                    stroke="none"
                >
                    <defs>
                        <rect
                            id="loader"
                            x="46.5"
                            y="40"
                            width="7"
                            height="20"
                            rx="2"
                            ry="2"
                            transform="translate(0 -30)"
                        />
                    </defs>
                    <use xlink:href="#loader" transform="rotate(30 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.08s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(60 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.17s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(90 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.25s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(120 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.33s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(150 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.42s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(180 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.50s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(210 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.58s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(240 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.67s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(270 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.75s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(300 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.83s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(330 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="0.92s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                    <use xlink:href="#loader" transform="rotate(360 50 50)">
                        <animateTransform
                            attributeName="transform"
                            type="scale"
                            additive="sum"
                            dur="1s"
                            begin="1.00s"
                            repeatCount="indefinite"
                            from="0"
                            to="1.2"
                        ></animateTransform>
                    </use>
                </svg>
            </section>
        </Show>
    }
}
