use crate::data::Data;
use sqlx::{MySql, Pool};

pub async fn create_data(pool: &Pool<MySql>, data: &Data) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO `data` (category_id, sub_category_id, tag_id, amount) VALUE (?, ?, ?, ?)",
    )
    .bind(data.category_id)
    .bind(data.sub_category_id)
    .bind(data.tag_id)
    .bind(data.amount)
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn modify_data(pool: &Pool<MySql>, id: i64, value: f64) -> anyhow::Result<()> {
    sqlx::query("UPDATE `data` SET `amount` = ? WHERE id = ?")
        .bind(value)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn remove_data(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM `data` WHERE `id` = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn list_data(pool: &Pool<MySql>, tag_id: Option<i64>) -> anyhow::Result<Vec<Data>> {
    let data = match tag_id {
        Some(tag_id) => sqlx::query_as::<_, Data>("SELECT `id`, `category_id`, `sub_category_id`, `tag_id`, `amount` FROM `data` WHERE `tag_id` = ?").bind(tag_id).fetch_all(pool).await?,
        None => sqlx::query_as::<_, Data>("SELECT `id`, `category_id`, `sub_category_id`, `tag_id`, `amount` FROM `data`").fetch_all(pool).await?,
    };
    Ok(data)
}
