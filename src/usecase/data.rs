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
        let data_list = self.repo.list_sub_category_data(tag_id).await?;
        let mut data_sub_category: HashMap<i64, SubCategoryData> = HashMap::new();
        for data in data_list {
            let Data {
                id,
                category_id,
                sub_category_id,
                amount,
                ..
            } = data;
            match data_sub_category.get_mut(&sub_category_id) {
                Some(previous) => {
                    previous.value += data.amount;
                }
                None => {
                    let sub_category = match sub_categories.get(&data.sub_category_id) {
                        Some(category) => category.clone(),
                        None => "".into(),
                    };
                    let data = SubCategoryData {
                        id: id.unwrap(),
                        category_id,
                        sub_category,
                        value: amount,
                    };
                    data_sub_category.insert(sub_category_id, data);
                }
            }
        }
        let mut result = data_sub_category
            .into_iter()
            .map(|data| data.1)
            .collect::<Vec<SubCategoryData>>();
        result.sort_by(|x, y| x.category_id.cmp(&y.category_id));
        Ok(result)
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
