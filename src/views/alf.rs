pub mod comp_view;
mod dashboard;
pub mod sales;

pub use dashboard::*;

use leptos::{html, prelude::*};

#[derive(Debug, Default, Clone)]
pub struct SalesDataWrapper {
    pub field: Vec<SalesData>,
}

#[derive(Debug, Clone, Default)]
pub struct SalesData {
    pub title: String,
    pub sales: Vec<Item>,
    pub total: f32,
}

#[derive(Clone, Debug, Default)]
pub struct Item {
    pub item_name: RwSignal<String>,
    pub qty: RwSignal<u32>,
    pub price: RwSignal<f32>,
    pub tax_group: RwSignal<f32>,
    pub focus_ref: NodeRef<html::Input>,
}

#[derive(Debug, Clone, Copy)]
pub enum TaxGroup {
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
