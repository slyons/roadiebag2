use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, FromRepr};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateUpdateItem {
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub size: ItemSize,
    pub infinite: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub size: ItemSize,
    pub infinite: bool,
}

#[derive(Serialize, Deserialize, FromRepr, EnumIter, Clone, Debug)]
#[repr(i16)]
pub enum ItemSize {
    Small = 0,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ItemFilter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub size: Option<ItemSize>,
    pub infinite: Option<bool>,
    pub page_num: u64,
    pub page_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemPage {
    pub items: Vec<Item>,
    pub page_num: u64,
    pub total_pages: u64,
    pub page_size: u64,
    pub total_results: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakenItem {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: i32,
    pub item_id: i32,
    pub rounds_left: i16,
    pub done: bool,
    pub rounds_total: i16,
}

#[derive(Serialize, Deserialize)]
pub struct TakenItemHistory {
    taken_history: Vec<TakenItem>,
}
