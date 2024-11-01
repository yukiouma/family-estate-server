use std::collections::HashMap;

use crate::{
    repo::Repo,
    service::data::{CategoryData, SubCategoryData},
};

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
    pub async fn list_sub_category_data(
        &self,
        tag_id: Option<i64>,
    ) -> anyhow::Result<Vec<SubCategoryData>> {
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
}

// { id: 0, categoryId: 1, subCategory: '活期存款', value: 300000 },
// { id: 0, categoryId: 2, subCategory: '汽车', value: 300000 },
// { id: 0, categoryId: 2, subCategory: '房子', value: 300000 },
// { id: 0, categoryId: 3, subCategory: '基金', value: 300000 },
// { id: 0, categoryId: 3, subCategory: '债券', value: 300000 },
// { id: 0, categoryId: 5, subCategory: '房贷', value: 300000 }
