use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CategoryData {
    pub id: i64,
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct SubCategoryData {
    pub id: i64,
    pub category_id: i64,
    pub sub_category: String,
    pub value: f64,
}
