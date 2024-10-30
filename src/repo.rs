use category::usecase::CategoryUsecase;
use data::usecase::DataUsecase;
use sqlx::MySqlPool;
use tag::usecase::TagUsecase;

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
}
