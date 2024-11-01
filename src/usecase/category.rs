use category::category::Category;

use crate::repo::Repo;

#[derive(Debug, Clone)]
pub struct CategoryUsecase {
    repo: Repo,
}

impl CategoryUsecase {
    pub fn new(repo: &Repo) -> Self {
        CategoryUsecase { repo: repo.clone() }
    }

    pub async fn list_category(&self) -> anyhow::Result<Vec<Category>> {
        self.repo.list_category().await
    }

    pub async fn list_sub_categories(&self, parent: Option<i64>) -> anyhow::Result<Vec<Category>> {
        self.repo.list_sub_categories(parent).await
    }

    pub async fn create_category(&self, category: &Category) -> anyhow::Result<()> {
        self.repo.create_category(&category).await
    }

    pub async fn create_sub_category(
        &self,
        parent: i64,
        category: &Category,
    ) -> anyhow::Result<()> {
        self.repo.create_sub_category(parent, category).await
    }

    pub async fn modify_category(&self, category: &Category) -> anyhow::Result<()> {
        self.repo.modify_category(category).await
    }

    pub async fn remove_category(&self, id: i64) -> anyhow::Result<()> {
        self.repo.remove_category(id).await
    }
}
