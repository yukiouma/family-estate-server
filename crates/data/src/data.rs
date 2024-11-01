use serde::Deserialize;
use sqlx::prelude::FromRow;

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
