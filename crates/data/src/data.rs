use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Data {
    pub id: Option<i64>,
    pub category_id: i64,
    pub sub_category_id: i64,
    pub tag_id: i64,
    pub amount: f64,
}

#[derive(Debug, Clone)]
pub struct CategoryData {
    pub category_id: i64,
    pub amount: f64,
}
