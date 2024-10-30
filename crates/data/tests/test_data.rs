use data::{data::Data, usecase::DataUsecase};
use sqlx::{MySql, MySqlPool, Pool};

#[tokio::test]
pub async fn data() -> anyhow::Result<()> {
    let url = "mysql://root:000000@localhost:3306/family-estate?parseTime=True";
    let pool = create_pool(url).await;
    let uc = DataUsecase::new(pool);
    uc.create_data(&Data {
        id: None,
        category_id: 1,
        sub_category_id: 2,
        tag_id: 1,
        amount: 100f64,
    })
    .await?;
    uc.create_data(&Data {
        id: None,
        category_id: 1,
        sub_category_id: 2,
        tag_id: 2,
        amount: 100f64,
    })
    .await?;
    let data_list = uc.list_data(None).await?;
    assert_eq!(2, data_list.len());
    assert_eq!(1, uc.list_data(Some(1)).await?.len());
    let category_data = uc.list_category_data(None).await?;
    assert_eq!(1, category_data.len());
    assert_eq!(200f64, category_data.first().unwrap().amount);
    uc.modify_data(data_list.first().unwrap().id.unwrap(), 500f64)
        .await?;
    let category_data = uc.list_category_data(None).await?;
    assert_eq!(600f64, category_data.first().unwrap().amount);
    uc.remove_data(data_list[0].id.unwrap()).await?;
    uc.remove_data(data_list[1].id.unwrap()).await?;
    assert_eq!(0, uc.list_data(None).await?.len());
    Ok(())
}

pub async fn create_pool(url: &str) -> Pool<MySql> {
    MySqlPool::connect(url)
        .await
        .expect("failed to connect database")
}
