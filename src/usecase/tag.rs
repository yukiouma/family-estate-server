use tag::tag::Tag;

use crate::repo::Repo;

#[derive(Debug, Clone)]
pub struct TagUsecase {
    repo: Repo,
}

impl TagUsecase {
    pub fn new(repo: &Repo) -> Self {
        TagUsecase { repo: repo.clone() }
    }
    pub async fn list_tags(&self) -> anyhow::Result<Vec<Tag>> {
        self.repo.list_tags().await
    }

    pub async fn create_tag(&self, tag: &Tag) -> anyhow::Result<()> {
        self.repo.create_tag(&tag).await
    }

    pub async fn modify_tag(&self, tag: &Tag) -> anyhow::Result<()> {
        self.repo.modify_tag(tag).await
    }

    pub async fn remove_tag(&self, id: i64) -> anyhow::Result<()> {
        self.repo.remove_tag(id).await
    }
}
