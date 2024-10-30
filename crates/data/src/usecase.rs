use sqlx::{MySql, Pool};

use crate::{
    data::{CategoryData, Data},
    repo::{create_data, list_data, modify_data, remove_data},
};

#[derive(Debug, Clone)]
pub struct DataUsecase {
    pool: Pool<MySql>,
}

impl DataUsecase {
    pub fn new(pool: Pool<MySql>) -> Self {
        DataUsecase { pool }
    }
    pub async fn create_data(&self, data: &Data) -> anyhow::Result<()> {
        create_data(&self.pool, data).await
    }
    pub async fn modify_data(&self, id: i64, amount: f64) -> anyhow::Result<()> {
        modify_data(&self.pool, id, amount).await
    }
    pub async fn remove_data(&self, id: i64) -> anyhow::Result<()> {
        remove_data(&self.pool, id).await
    }
    pub async fn list_data(&self, tag_id: Option<i64>) -> anyhow::Result<Vec<Data>> {
        list_data(&self.pool, tag_id).await
    }
    pub async fn list_category_data(
        &self,
        tag_id: Option<i64>,
    ) -> anyhow::Result<Vec<CategoryData>> {
        let mut result = vec![];
        let mut data = list_data(&self.pool, tag_id).await?;
        if data.is_empty() {
            return Ok(result);
        }
        data.sort_by_key(|data| data.category_id);
        let mut category_data = CategoryData {
            category_id: data.first().unwrap().category_id,
            amount: 0f64,
        };
        for row in data {
            if row.category_id.eq(&category_data.category_id) {
                category_data.amount = category_data.amount + row.amount;
            } else {
                result.push(category_data.clone());
                category_data.category_id = row.category_id;
                category_data.amount = row.amount;
            }
        }
        result.push(category_data);
        Ok(result)
    }
}
