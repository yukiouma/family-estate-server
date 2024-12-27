use crate::repo::Repo;
use statistic::statistic::{CreateStatistic, Statistic, StatisticCategory};

#[derive(Debug, Clone)]
pub struct StatisticUsecase {
    repo: Repo,
}

impl StatisticUsecase {
    pub fn new(repo: &Repo) -> Self {
        StatisticUsecase { repo: repo.clone() }
    }
    pub async fn list_statistic(&self) -> anyhow::Result<Vec<Statistic>> {
        self.repo.list_statistic().await
    }
    pub async fn get_statistic_category(
        &self,
        id: i64,
    ) -> anyhow::Result<Option<StatisticCategory>> {
        self.repo.get_statistic_category(id).await
    }
    pub async fn create_statistic(&self, statistic: &CreateStatistic) -> anyhow::Result<()> {
        self.repo.create_statistic(statistic).await
    }
    pub async fn modify_statistic(&self, id: i64, name: &str) -> anyhow::Result<()> {
        self.repo.modify_statistic(id, name).await
    }
    pub async fn remove_statistic(&self, id: i64) -> anyhow::Result<()> {
        self.repo.remove_statistic(id).await
    }
}
