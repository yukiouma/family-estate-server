use sqlx::{MySql, MySqlPool, Pool};
use statistic::{statistic::CreateStatistic, usecase::StatisticUsecase};

#[tokio::test]
pub async fn statistic_test() -> anyhow::Result<()> {
    let url = "mysql://root:000000@localhost:3306/family-estate?parseTime=True";
    let pool = create_pool(url).await;
    let uc = StatisticUsecase::new(pool);
    let list = uc.list_statistics().await?;
    assert_eq!(0, list.len());
    uc.create_statistic(&CreateStatistic {
        name: "cashier".to_string(),
        categories: vec![1],
    })
    .await?;
    let list = uc.list_statistics().await?;
    assert_eq!(1, list.len());
    let category = uc.get_statistic_category(list[0].id).await?.unwrap();
    assert_eq!(1, category.category_name.len());
    uc.modify_statistic(list[0].id, "cashier2").await?;
    let list = uc.list_statistics().await?;
    assert_eq!(1, list.len());
    uc.remove_statistic(list[0].id).await?;
    let list = uc.list_statistics().await?;
    assert_eq!(0, list.len());
    Ok(())
}

pub async fn create_pool(url: &str) -> Pool<MySql> {
    MySqlPool::connect(url)
        .await
        .expect("failed to connect database")
}
