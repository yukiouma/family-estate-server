use serde::Serialize;
use sqlx::FromRow;

pub struct CreateStatistic {
    pub name: String,
    pub categories: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct Statistic {
    pub id: i64,
    #[serde(rename(serialize = "total"))]
    pub total_amount: f64,
    pub label: String,
    #[serde(rename(serialize = "byTag"))]
    pub by_tag: Vec<StatisticResultByTag>,
}

#[derive(Debug, Serialize)]
pub struct StatisticResultByTag {
    #[serde(rename(serialize = "id"))]
    pub tag_id: i64,
    pub amount: f64,
    #[serde(rename(serialize = "name"))]
    pub tag: String,
}

#[derive(Debug, FromRow)]
pub struct StatisticRow {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow)]
pub struct StatisticCategoryRow {
    pub id: i64,
    pub label: String,
    pub category_name: String,
}

#[derive(Debug, FromRow)]
pub struct StatisticResult {
    pub amount: f64,
    pub tag_id: i64,
    pub statistic_id: i64,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct StatisticCategory {
    pub id: i64,
    #[serde(rename(serialize = "name"))]
    pub label: String,
    #[serde(rename(serialize = "categories"))]
    pub category_name: Vec<String>,
}
