use category::{category::Category, usecase::CategoryUsecase};
use sqlx::{MySql, MySqlPool, Pool};

#[tokio::test]
async fn category() -> anyhow::Result<()> {
    let url = "mysql://root:000000@localhost:3306/family-estate?parseTime=True";
    let pool = create_pool(url).await;
    let uc = CategoryUsecase::new(pool);
    uc.add_category(&Category {
        id: None,
        name: "main_demo".into(),
    })
    .await?;
    let mut categories = uc.list_categories().await?;
    assert_eq!(categories.len(), 1);
    let category = categories.first_mut().unwrap();
    uc.add_sub_category(
        category.id.unwrap(),
        &Category {
            id: None,
            name: "sub_demo".into(),
        },
    )
    .await?;
    let mut sub_categories = uc.list_sub_categories(category.id.unwrap()).await?;
    assert_eq!(sub_categories.len(), 1);
    let sub_category = sub_categories.first_mut().unwrap();
    category.name = "main_update".into();
    uc.modify_category(&category).await?;
    assert_eq!(
        uc.list_categories().await?.first().unwrap().name,
        "main_update"
    );
    sub_category.name = "sub_update".into();
    uc.modify_category(&sub_category).await?;
    assert_eq!(
        uc.list_sub_categories(category.id.unwrap())
            .await?
            .first()
            .unwrap()
            .name,
        "sub_update"
    );
    uc.remove_category(category.id.unwrap()).await?;
    uc.remove_category(sub_category.id.unwrap()).await?;
    assert_eq!(uc.list_categories().await?.len(), 0);
    assert_eq!(uc.list_sub_categories(category.id.unwrap()).await?.len(), 0);
    Ok(())
}

pub async fn create_pool(url: &str) -> Pool<MySql> {
    MySqlPool::connect(url)
        .await
        .expect("failed to connect database")
}
