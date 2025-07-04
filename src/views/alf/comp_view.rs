use leptos::prelude::*;

use crate::views::alf::sales::{SalesCreateView, SalesDisplayView};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentView {
    #[default]
    Display,
    Create,
}

impl ComponentView {
    pub fn to_view(&self) -> impl IntoView {
        match self {
            ComponentView::Create => view! { <SalesCreateView /> }.into_any(),
            ComponentView::Display => view! { <SalesDisplayView /> }.into_any(),
        }
    }
}
