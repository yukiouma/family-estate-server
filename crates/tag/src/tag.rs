use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
}
