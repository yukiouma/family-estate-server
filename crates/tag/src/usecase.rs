use sqlx::{MySql, Pool};

use crate::{
    repo::{create_tag, list_tags, modify_tag, remove_tag},
    tag::Tag,
};

#[derive(Debug, Clone)]
pub struct TagUsecase {
    pool: Pool<MySql>,
}

impl TagUsecase {
    pub fn new(pool: Pool<MySql>) -> Self {
        TagUsecase { pool }
    }
    pub async fn list_tag(&self) -> anyhow::Result<Vec<Tag>> {
        let tag = list_tags(&self.pool).await?;
        Ok(tag)
    }
    pub async fn create_tag(&self, tag: &Tag) -> anyhow::Result<()> {
        create_tag(&self.pool, tag).await?;
        Ok(())
    }
    pub async fn modify_tag(&self, tag: &Tag) -> anyhow::Result<()> {
        modify_tag(&self.pool, tag).await?;
        Ok(())
    }
    pub async fn remove_tag(&self, id: i64) -> anyhow::Result<()> {
        remove_tag(&self.pool, id).await?;
        Ok(())
    }
}
