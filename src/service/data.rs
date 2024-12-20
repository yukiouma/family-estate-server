use data::data::HistoryData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CategoryData {
    pub id: i64,
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct SubCategoryData {
    pub id: i64,
    #[serde(rename(serialize = "categoryId"))]
    pub category_id: i64,
    #[serde(rename(serialize = "subCategory", deserialize = "subCategory"))]
    pub sub_category: String,
    #[serde(rename(serialize = "subCategoryId"))]
    pub sub_category_id: i64,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
pub struct ListCategoryDataRequest {
    pub tag_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListCategoryDataReply {
    pub data: Vec<CategoryData>,
}

#[derive(Debug, Deserialize)]
pub struct ListDataRequest {
    pub tag_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListDataReply {
    pub data: Vec<SubCategoryData>,
}

#[derive(Debug, Deserialize)]
pub struct ListHistoryRequest {
    pub record_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ListHistoryReply {
    pub data: Vec<HistoryData>,
}

#[derive(Debug, Deserialize)]
pub struct ListCategoryHistoryRequest {
    pub sub_category_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ListCategoryHistoryReply {
    pub data: Vec<HistoryData>,
}

#[derive(Debug, Serialize)]
pub struct CreateDataReply {}

#[derive(Debug, Deserialize)]
pub struct ModifyDataRequest {
    pub amount: f64,
}

#[derive(Debug, Serialize)]
pub struct ModifyDataReply {}

#[derive(Debug, Serialize)]
pub struct RemoveDataReply {}
