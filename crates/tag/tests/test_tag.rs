use tag::{tag::Tag, usecase::TagUsecase};
use sqlx::{MySql, MySqlPool, Pool};

#[tokio::test]
async fn tag() {
    let url = "mysql://root:000000@localhost:3306/family-estate?parseTime=True";
    let pool = create_pool(url).await;
    let usecase = TagUsecase::new(pool);
    usecase
        .create_tag(&Tag {
            id: None,
            name: "yuki".into(),
        })
        .await
        .expect("failed to create tag");
    let mut tags = usecase.list_tag().await.expect("failed to list tags");
    assert_eq!(tags.len(), 1);
    let tag = tags.get_mut(0).unwrap();
    assert_eq!(tag.name, "yuki");
    tag.name = "malney".into();
    usecase.modify_tag(tag).await.expect("failed to modify tag");
    let tags = usecase.list_tag().await.expect("failed to list tags");
    let tag = tags.get(0).unwrap();
    assert_eq!(tag.name, "malney");
    usecase
        .remove_tag(tag.id.unwrap())
        .await
        .expect("failed to remove tag");
}




pub async fn create_pool(url: &str) -> Pool<MySql> {
    MySqlPool::connect(url)
        .await
        .expect("failed to connect database")
}
