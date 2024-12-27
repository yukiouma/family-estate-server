use crate::{
    repo::{
        create_statistic, get_statistic_category, list_statistic_by_tag, list_statistic_total,
        list_tags, modify_statistic, remove_statistic,
    },
    statistic::{
        CreateStatistic, Statistic, StatisticCategory, StatisticResult, StatisticResultByTag,
    },
};
use sqlx::{MySql, Pool};

#[derive(Debug, Clone)]
pub struct StatisticUsecase {
    pool: Pool<MySql>,
}

impl StatisticUsecase {
    pub fn new(pool: Pool<MySql>) -> Self {
        StatisticUsecase { pool }
    }

    pub async fn list_statistics(&self) -> anyhow::Result<Vec<Statistic>> {
        let tags = list_tags(&self.pool).await?;
        let total = list_statistic_total(&self.pool).await?;
        let mut by_tags = Vec::with_capacity(tags.len());
        for tag in tags.iter() {
            let result = list_statistic_by_tag(&self.pool, tag.id).await?;
            by_tags.push(result);
        }
        let mut result = Vec::with_capacity(total.len());
        for t in total {
            let stat = Statistic {
                id: t.statistic_id,
                total_amount: t.amount,
                label: t.label,
                by_tag: tags
                    .iter()
                    .enumerate()
                    .map(|(index, tag)| {
                        let by_tag = by_tags[index]
                            .iter()
                            .filter(|r| r.statistic_id == t.statistic_id)
                            .collect::<Vec<&StatisticResult>>();
                        StatisticResultByTag {
                            tag_id: tag.id,
                            amount: if by_tag.is_empty() {
                                0.0
                            } else {
                                by_tag[0].amount
                            },
                            tag: tag.name.clone(),
                        }
                    })
                    .collect(),
            };
            result.push(stat);
        }

        Ok(result)
    }

    pub async fn get_statistic_category(
        &self,
        id: i64,
    ) -> anyhow::Result<Option<StatisticCategory>> {
        let rows = get_statistic_category(&self.pool, id).await?;
        if rows.is_empty() {
            Ok(None)
        } else {
            Ok(Some(StatisticCategory {
                id: rows[0].id,
                label: rows[0].label.clone(),
                category_name: rows.iter().map(|r| r.category_name.clone()).collect(),
            }))
        }
    }

    pub async fn create_statistic(&self, statistic: &CreateStatistic) -> anyhow::Result<()> {
        create_statistic(&self.pool, statistic).await
    }

    pub async fn modify_statistic(&self, id: i64, name: &str) -> anyhow::Result<()> {
        modify_statistic(&self.pool, id, name).await
    }

    pub async fn remove_statistic(&self, id: i64) -> anyhow::Result<()> {
        remove_statistic(&self.pool, id).await
    }
}
