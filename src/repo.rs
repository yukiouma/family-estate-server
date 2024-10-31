use category::{category::Category, usecase::CategoryUsecase};
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
    pub async fn list_category(&self) -> anyhow::Result<Vec<Category>> {
        self.category.list_categories().await
    }
    pub async fn list_sub_categories(&self, parent: i64) -> anyhow::Result<Vec<Category>> {
        self.category.list_sub_categories(parent).await
    }
    pub async fn create_category(&self, category: &Category) -> anyhow::Result<()> {
        self.category.add_category(category).await
    }
    pub async fn create_sub_category(
        &self,
        parent: i64,
        category: &Category,
    ) -> anyhow::Result<()> {
        self.category.add_sub_category(parent, category).await
    }
    pub async fn modify_category(&self, category: &Category) -> anyhow::Result<()> {
        self.category.modify_category(category).await
    }
    pub async fn remove_category(&self, category_id: i64) -> anyhow::Result<()> {
        self.category.remove_category(category_id).await
    }
}
