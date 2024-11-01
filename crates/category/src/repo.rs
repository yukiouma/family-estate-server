use sqlx::{MySql, Pool};

use crate::category::Category;

pub async fn list_categories(pool: &Pool<MySql>) -> anyhow::Result<Vec<Category>> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT `id`, `name` FROM `category` WHERE `parent_id` = 0 AND `deleted_at` IS NULL",
    )
    .fetch_all(pool)
    .await?;
    Ok(categories)
}

pub async fn list_sub_categories(
    pool: &Pool<MySql>,
    parent: Option<i64>,
) -> anyhow::Result<Vec<Category>> {
    let sub_categories = match parent {
        Some(parent) => sqlx::query_as::<_, Category>(
            "SELECT `id`, `name` FROM `category` WHERE `parent_id` = ? AND `deleted_at` IS NULL",
        )
        .bind(parent)
        .fetch_all(pool)
        .await?,
        None => {
            sqlx::query_as::<_, Category>(
                "SELECT `id`, `name` FROM `category` WHERE `deleted_at` IS NULL",
            )
            .fetch_all(pool)
            .await?
        }
    };
    Ok(sub_categories)
}

pub async fn add_category(pool: &Pool<MySql>, category: &Category) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO `category` (`name`) VALUE (?)")
        .bind(category.name.to_owned())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_sub_category(
    pool: &Pool<MySql>,
    parent: i64,
    category: &Category,
) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO `category` (`name`, `parent_id`) VALUE (?, ?)")
        .bind(category.name.to_owned())
        .bind(parent)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn modify_category(pool: &Pool<MySql>, category: &Category) -> anyhow::Result<()> {
    if let Some(id) = category.id {
        sqlx::query("UPDATE `category` SET `name` = ? WHERE `id` = ?")
            .bind(category.name.to_owned())
            .bind(id)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn remove_category(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM `category` WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
