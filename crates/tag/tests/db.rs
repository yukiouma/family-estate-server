use sqlx::{MySql, MySqlPool, Pool};

pub async fn create_pool(url: &str) -> Pool<MySql> {
    MySqlPool::connect(url)
        .await
        .expect("failed to connect database")
}
