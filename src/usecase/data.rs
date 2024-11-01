use std::collections::HashMap;

use data::data::Data;

use crate::{
    repo::Repo,
    service::data::{CategoryData, SubCategoryData},
};

#[derive(Debug, Clone)]
pub struct DataUsecase {
    repo: Repo,
}

impl DataUsecase {
    pub fn new(repo: &Repo) -> Self {
        DataUsecase { repo: repo.clone() }
    }

    pub async fn list_category_data(
        &self,
        tag_id: Option<i64>,
    ) -> anyhow::Result<Vec<CategoryData>> {
        let data = self.repo.list_category_data(tag_id).await?;
        let categorys = self
            .repo
            .list_category()
            .await?
            .into_iter()
            .map(|category| (category.id.unwrap(), category.name))
            .collect::<HashMap<i64, String>>();
        Ok(data
            .into_iter()
            .map(|data| {
                let name = match categorys.get(&data.category_id) {
                    Some(name) => name.into(),
                    None => "".into(),
                };
                CategoryData {
                    id: data.category_id,
                    name,
                    value: data.amount,
                }
            })
            .collect())
    }
    pub async fn list_data(&self, tag_id: Option<i64>) -> anyhow::Result<Vec<SubCategoryData>> {
        let sub_categories = self
            .repo
            .list_sub_categories(None)
            .await?
            .into_iter()
            .map(|category| (category.id.unwrap(), category.name))
            .collect::<HashMap<i64, String>>();
        Ok(self
            .repo
            .list_sub_category_data(tag_id)
            .await?
            .into_iter()
            .map(|data| {
                let sub_category = match sub_categories.get(&data.sub_category_id) {
                    Some(category) => category.clone(),
                    None => "".into(),
                };
                SubCategoryData {
                    id: data.id.unwrap(),
                    category_id: data.category_id,
                    sub_category,
                    value: data.amount,
                }
            })
            .collect())
    }

    pub async fn create_data(&self, data: &Data) -> anyhow::Result<()> {
        self.repo.create_data(data).await
    }

    pub async fn modify_data(&self, id: i64, amount: f64) -> anyhow::Result<()> {
        self.repo.modify_data(id, amount).await
    }

    pub async fn remove_data(&self, id: i64) -> anyhow::Result<()> {
        self.repo.remove_data(id).await
    }
}
