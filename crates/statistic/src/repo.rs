use crate::statistic::{CreateStatistic, StatisticCategoryRow, StatisticResult, Tag};
use sqlx::{MySql, Pool};

pub async fn create_statistic(
    pool: &Pool<MySql>,
    statistic: &CreateStatistic,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let result = sqlx::query("INSERT INTO `statistic` (label) VALUE (?)")
        .bind(&statistic.name)
        .execute(&mut *tx)
        .await?;
    let id = result.last_insert_id();
    let mut query =
        String::from("INSERT INTO `statistic_category` (`statistic_id`, `category_id`) VALUES ");
    let mut params = Vec::new();
    for (i, category_id) in statistic.categories.iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }
        query.push_str("(?, ?)");
        params.push((id, category_id));
    }
    let mut query = sqlx::query(&query);
    for category_id in statistic.categories.iter() {
        query = query.bind(id).bind(category_id);
    }
    query.execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn modify_statistic(pool: &Pool<MySql>, id: i64, name: &str) -> anyhow::Result<()> {
    sqlx::query("UPDATE `statistic` SET `label` = ? WHERE `id` = ?")
        .bind(name)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn remove_statistic(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM `statistic` WHERE `id` = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM `statistic_category` WHERE `statistic_id` = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn list_statistic_total(pool: &Pool<MySql>) -> anyhow::Result<Vec<StatisticResult>> {
    // total
    let result = sqlx::query_as::<_, StatisticResult>(
        "
SELECT SUM(`data`.`amount`) AS `amount`,
    0 AS `tag_id`,
    `statistic_category`.`statistic_id`,
    `statistic`.`label` AS `label`
FROM `statistic_category`
    LEFT JOIN `data` ON `statistic_category`.`category_id` = `data`.`category_id`
    LEFT JOIN `statistic` ON `statistic_category`.`statistic_id` = `statistic`.`id`
GROUP BY `statistic_category`.`statistic_id`
ORDER BY `statistic`.`id`;
    ",
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn list_statistic_by_tag(
    pool: &Pool<MySql>,
    tag_id: i64,
) -> anyhow::Result<Vec<StatisticResult>> {
    // total
    let result = sqlx::query_as::<_, StatisticResult>(
        "
SELECT SUM(`data`.`amount`) AS `amount`,
    0 AS `tag_id`,
    `statistic_category`.`statistic_id`,
    `statistic`.`label` AS `label`
FROM `statistic_category`
    LEFT JOIN `data` ON `statistic_category`.`category_id` = `data`.`category_id`
    LEFT JOIN `statistic` ON `statistic_category`.`statistic_id` = `statistic`.`id`
WHERE `data`.`tag_id` = ?
GROUP BY `statistic_category`.`statistic_id`;
    ",
    )
    .bind(tag_id)
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn get_statistic_category(
    pool: &Pool<MySql>,
    id: i64,
) -> anyhow::Result<Vec<StatisticCategoryRow>> {
    let statistic = sqlx::query_as::<_, StatisticCategoryRow>(
        "
SELECT `statistic`.`id` AS `id`,
    `statistic`.`label` AS `label`,
    `category`.`name` AS `category_name`
FROM `statistic_category`
    LEFT JOIN `category` ON `category`.`id` = `statistic_category`.`category_id`
    LEFT JOIN `statistic` ON `statistic`.`id` = `statistic_category`.`statistic_id`
WHERE `statistic_id` = ?;
        ",
    )
    .bind(id)
    .fetch_all(pool)
    .await?;
    Ok(statistic)
}

pub async fn list_tags(pool: &Pool<MySql>) -> anyhow::Result<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM `tag`")
        .fetch_all(pool)
        .await?;
    Ok(tags)
}
