use sqlx::{MySql, Pool};

use crate::tag::Tag;

pub async fn list_tags(pool: &Pool<MySql>) -> anyhow::Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>("SELECT id, name FROM tag WHERE deleted_at IS NULL")
        .fetch_all(pool)
        .await?;
    Ok(tags)
}

pub async fn create_tag(pool: &Pool<MySql>, tag: &Tag) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO tag (name) VALUE (?)")
        .bind(tag.name.to_owned())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn modify_tag(pool: &Pool<MySql>, tag: &Tag) -> anyhow::Result<()> {
    if let Some(id) = tag.id {
        sqlx::query("UPDATE tag SET name = ? WHERE id = ?")
            .bind(tag.name.to_owned())
            .bind(id)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn remove_tag(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM tag WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
