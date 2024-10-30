use sqlx::{MySql, Pool};

use crate::category::Category;
use crate::repo::{
    add_category, add_sub_category, list_categories, list_sub_categories, modify_category,
    remove_category,
};

#[derive(Debug, Clone)]
pub struct CategoryUsecase {
    pool: Pool<MySql>,
}

impl CategoryUsecase {
    pub fn new(pool: Pool<MySql>) -> Self {
        CategoryUsecase { pool }
    }

    pub async fn list_categories(&self) -> anyhow::Result<Vec<Category>> {
        Ok(list_categories(&self.pool).await?)
    }

    pub async fn list_sub_categories(&self, parent: i64) -> anyhow::Result<Vec<Category>> {
        Ok(list_sub_categories(&self.pool, parent).await?)
    }

    pub async fn add_category(&self, category: &Category) -> anyhow::Result<()> {
        Ok(add_category(&self.pool, category).await?)
    }

    pub async fn add_sub_category(&self, parent: i64, category: &Category) -> anyhow::Result<()> {
        Ok(add_sub_category(&self.pool, parent, category).await?)
    }

    pub async fn modify_category(&self, category: &Category) -> anyhow::Result<()> {
        Ok(modify_category(&self.pool, category).await?)
    }

    pub async fn remove_category(&self, id: i64) -> anyhow::Result<()> {
        Ok(remove_category(&self.pool, id).await?)
    }
}
