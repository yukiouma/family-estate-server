use category::{category::Category, usecase::CategoryUsecase};
use data::{
    data::{CategoryData, Data, HistoryData},
    usecase::DataUsecase,
};
use sqlx::MySqlPool;
use statistic::{
    statistic::{CreateStatistic, Statistic, StatisticCategory},
    usecase::StatisticUsecase,
};
use tag::{tag::Tag, usecase::TagUsecase};

#[derive(Debug, Clone)]
pub struct Repo {
    tag: TagUsecase,
    data: DataUsecase,
    category: CategoryUsecase,
    statistic: StatisticUsecase,
}

impl Repo {
    pub async fn new(database_url: &str) -> anyhow::Result<Repo> {
        let pool = MySqlPool::connect(database_url).await?;
        let tag = TagUsecase::new(pool.clone());
        let data = DataUsecase::new(pool.clone());
        let category = CategoryUsecase::new(pool.clone());
        let statistic = StatisticUsecase::new(pool);
        Ok(Repo {
            tag,
            data,
            category,
            statistic,
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
    pub async fn list_sub_categories(&self, parent: Option<i64>) -> anyhow::Result<Vec<Category>> {
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
    pub async fn list_category_data(
        &self,
        tag_id: Option<i64>,
    ) -> anyhow::Result<Vec<CategoryData>> {
        self.data.list_category_data(tag_id).await
    }
    pub async fn list_sub_category_data(&self, tag_id: Option<i64>) -> anyhow::Result<Vec<Data>> {
        self.data.list_data(tag_id).await
    }
    pub async fn list_history(&self, record_id: i64) -> anyhow::Result<Vec<HistoryData>> {
        self.data.list_history(record_id).await
    }
    pub async fn list_category_history(
        &self,
        sub_category_id: i64,
    ) -> anyhow::Result<Vec<HistoryData>> {
        self.data.list_category_history(sub_category_id).await
    }
    pub async fn create_data(&self, data: &Data) -> anyhow::Result<()> {
        self.data.create_data(data).await
    }
    pub async fn modify_data(&self, id: i64, amount: f64) -> anyhow::Result<()> {
        self.data.modify_data(id, amount).await
    }
    pub async fn remove_data(&self, id: i64) -> anyhow::Result<()> {
        self.data.remove_data(id).await
    }
    pub async fn list_statistic(&self) -> anyhow::Result<Vec<Statistic>> {
        self.statistic.list_statistics().await
    }
    pub async fn get_statistic_category(
        &self,
        id: i64,
    ) -> anyhow::Result<Option<StatisticCategory>> {
        self.statistic.get_statistic_category(id).await
    }
    pub async fn create_statistic(&self, statistic: &CreateStatistic) -> anyhow::Result<()> {
        self.statistic.create_statistic(statistic).await
    }
    pub async fn modify_statistic(&self, id: i64, name: &str) -> anyhow::Result<()> {
        self.statistic.modify_statistic(id, name).await
    }
    pub async fn remove_statistic(&self, id: i64) -> anyhow::Result<()> {
        self.statistic.remove_statistic(id).await
    }
}
