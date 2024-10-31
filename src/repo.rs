use category::usecase::CategoryUsecase;
use data::usecase::DataUsecase;
use sqlx::MySqlPool;
use tag::{tag::Tag, usecase::TagUsecase};

#[derive(Debug, Clone)]
pub struct Repo {
    tag: TagUsecase,
    data: DataUsecase,
    category: CategoryUsecase,
}

impl Repo {
    pub async fn new(database_url: &str) -> anyhow::Result<Repo> {
        let pool = MySqlPool::connect(database_url).await?;
        let tag = TagUsecase::new(pool.clone());
        let data = DataUsecase::new(pool.clone());
        let category = CategoryUsecase::new(pool);
        Ok(Repo {
            tag,
            data,
            category,
        })
    }
    pub async fn list_tags(&self) -> anyhow::Result<Vec<Tag>> {
        self.tag.list_tag().await
    }
    pub async fn create_tag(&self, tag: &Tag) -> anyhow::Result<()> {
        self.tag.create_tag(tag).await
    }
    pub async fn modify_tag(&self, tag: &Tag) -> anyhow::Result<()> {
        self.tag.modify_tag(tag).await
    }
    pub async fn remove_tag(&self, id: i64) -> anyhow::Result<()> {
        self.tag.remove_tag(id).await
    }
}
