use category::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ListCategoryReply {
    pub data: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct ListSubCategoryRequest {
    pub category_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateCategroyReply {}

#[derive(Debug, Deserialize)]
pub struct CreateSubCategroyRequest {
    pub parent: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSubCategroyReply {}

#[derive(Debug, Serialize)]
pub struct ModifyCategroyReply {}

#[derive(Debug, Serialize)]
pub struct RemoveCategoryReply {}
