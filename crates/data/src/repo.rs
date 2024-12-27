use crate::data::{Data, HistoryRow};
use sqlx::{types::chrono, MySql, Pool};

pub async fn create_data(pool: &Pool<MySql>, data: &Data) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let result = sqlx::query(
        "INSERT INTO `data` (category_id, sub_category_id, tag_id, amount) VALUE (?, ?, ?, ?)",
    )
    .bind(data.category_id)
    .bind(data.sub_category_id)
    .bind(data.tag_id)
    .bind(data.amount)
    .execute(&mut *tx)
    .await?;
    let id = result.last_insert_id();
    let now = format!("{} 08:00:00", chrono::Local::now().format("%Y-%m-%d"));
    sqlx::query("INSERT INTO `history` (`data_id`, `amount`, `history_date`) VALUE (?, ?, ?)")
        .bind(id)
        .bind(data.amount)
        .bind(now)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
pub async fn modify_data(pool: &Pool<MySql>, id: i64, value: f64) -> anyhow::Result<()> {
    let now = format!("{} 08:00:00", chrono::Local::now().format("%Y-%m-%d"));
    let mut tx = pool.begin().await?;
    let result = sqlx::query("UPDATE `data` SET `amount` = ? WHERE id = ?")
        .bind(value)
        .bind(id)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() > 0 {
        let history = sqlx::query_as::<_, HistoryRow>(
            "SELECT `id`, `data_id`, `amount`, `history_date` FROM `history` WHERE `data_id` = ? AND `history_date` = ?",
        )
        .bind(id)
        .bind(&now)
        .fetch_optional(pool)
        .await?;
        if let Some(history) = history {
            sqlx::query("UPDATE `history` SET `amount` = ? WHERE `id` = ?")
                .bind(value)
                .bind(history.id)
                .execute(&mut *tx)
                .await?;
        } else {
            sqlx::query(
                "INSERT INTO `history` (`data_id`, `amount`, `history_date`) VALUE (?, ?, ?)",
            )
            .bind(id)
            .bind(value)
            .bind(&now)
            .execute(&mut *tx)
            .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}
pub async fn remove_data(pool: &Pool<MySql>, id: i64) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM `data` WHERE `id` = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM `history` WHERE `data_id` = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}
pub async fn list_data(pool: &Pool<MySql>, tag_id: Option<i64>) -> anyhow::Result<Vec<Data>> {
    let data = match tag_id {
        Some(tag_id) => sqlx::query_as::<_, Data>("SELECT `id`, `category_id`, `sub_category_id`, `tag_id`, `amount` FROM `data` WHERE `tag_id` = ?").bind(tag_id).fetch_all(pool).await?,
        None => sqlx::query_as::<_, Data>("SELECT `id`, `category_id`, `sub_category_id`, `tag_id`, `amount` FROM `data`").fetch_all(pool).await?,
    };
    Ok(data)
}
pub async fn list_history(pool: &Pool<MySql>, record_id: i64) -> anyhow::Result<Vec<HistoryRow>> {
    let data = sqlx::query_as::<_, HistoryRow>(
        "SELECT `id`, `data_id`, `amount`, `history_date` FROM `history` WHERE `data_id` = ?",
    )
    .bind(record_id)
    .fetch_all(pool)
    .await?;
    Ok(data)
}

pub async fn list_cateory_history(
    pool: &Pool<MySql>,
    sub_category_id: i64,
) -> anyhow::Result<Vec<HistoryRow>> {
    let record_ids = sqlx::query_as::<_, Data>("SELECT `id`, `category_id`, `sub_category_id`, `tag_id`, `amount` FROM `data` WHERE `sub_category_id` = ?")
        .bind(sub_category_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|d|d.id.unwrap())
        .collect::<Vec<i64>>();
    let syntax = format!(
        "SELECT 0 AS `id`, SUM(`amount`) AS `amount`, `history_date` FROM `history` WHERE `data_id` in ({}) GROUP BY `history_date`",
        record_ids.iter().map(|_|"?").collect::<Vec<&str>>().join(",")
     );
    let mut querier = sqlx::query_as::<_, HistoryRow>(&syntax);
    for id in record_ids {
        querier = querier.bind(id);
    }
    let data = querier.fetch_all(pool).await?;
    Ok(data)
}
