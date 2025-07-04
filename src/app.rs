use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment,
};

use crate::{layouts, views};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/sales-inventory.css" />

        // sets the document title
        <Title text="Welcome to Alfio's Inventory" />

        // content for this welcome page
        <Router>
            <main class="min-h-screen">
                <Routes fallback=views::NotFoundView>
                    <Route path=StaticSegment("") view=views::LoginView />
                    <ParentRoute path=StaticSegment("home") view=layouts::MainLayout>
                        <Route path=StaticSegment("") view=views::alf::DashboardALF />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
