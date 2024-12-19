use std::ops::{Div, Sub};

use sqlx::{MySql, Pool};

use crate::{
    data::{CategoryData, Data, HistoryData, HistoryRow},
    repo::{create_data, list_cateory_history, list_data, list_history, modify_data, remove_data},
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

    pub async fn list_history(&self, record_id: i64) -> anyhow::Result<Vec<HistoryData>> {
        let rows = list_history(&self.pool, record_id).await?;
        Ok(history_row_to_data(&rows))
    }

    pub async fn list_category_history(
        &self,
        sub_category_id: i64,
    ) -> anyhow::Result<Vec<HistoryData>> {
        let rows = list_cateory_history(&self.pool, sub_category_id).await?;
        Ok(history_row_to_data(&rows))
    }
}

fn history_row_to_data(rows: &[HistoryRow]) -> Vec<HistoryData> {
    let mut result: Vec<HistoryData> = Vec::with_capacity(rows.len());
    let mut min = 0f64;
    let mut max = 0f64;
    let mut min_index = 0;
    let mut max_index = 0;
    let mut last_amount: Option<f64> = None;
    for (index, row) in rows.into_iter().enumerate() {
        let date = row.history_date.format("%Y-%m-%d").to_string();
        if let Some(last) = last_amount {
            let change = last.sub(&row.amount).div(&last);
            last_amount = Some(row.amount);
            result.push(HistoryData {
                id: row.id,
                amount: row.amount,
                change,
                date: date.clone(),
                is_max: false,
                is_min: false,
            });
        } else {
            result.push(HistoryData {
                id: row.id,
                amount: row.amount,
                change: 0f64,
                date: date.clone(),
                is_max: false,
                is_min: false,
            });
        }
        if row.amount.gt(&max) {
            max = row.amount;
            max_index = index
        } else if row.amount.lt(&min) {
            min = row.amount;
            min_index = index;
        }
    }
    if let Some(history) = result.get_mut(max_index) {
        history.is_max = true;
    }
    if let Some(history) = result.get_mut(min_index) {
        history.is_min = true;
    }
    result
}
