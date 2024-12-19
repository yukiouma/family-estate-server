use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::chrono::{self, Local},
};

#[derive(Debug, FromRow, Deserialize)]
pub struct Data {
    pub id: Option<i64>,
    #[serde(rename(deserialize = "categoryId"))]
    pub category_id: i64,
    #[serde(rename(deserialize = "subCategoryId"))]
    pub sub_category_id: i64,
    #[serde(rename(deserialize = "tagId"))]
    pub tag_id: i64,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct CategoryData {
    pub category_id: i64,
    pub amount: f64,
}

#[derive(Debug, FromRow)]
pub struct HistoryRow {
    pub id: i64,
    pub amount: f64,
    pub history_date: chrono::DateTime<Local>,
}

#[derive(Debug, Serialize)]
pub struct HistoryData {
    pub id: i64,
    pub amount: f64,
    pub date: String,
    pub change: f64,
    #[serde(rename(serialize = "isMin"))]
    pub is_min: bool,
    #[serde(rename(serialize = "isMax"))]
    pub is_max: bool,
}
