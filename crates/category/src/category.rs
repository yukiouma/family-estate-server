use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Category {
    pub id: Option<i64>,
    pub name: String,
}
