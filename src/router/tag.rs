use axum::{extract::State, routing::get, Json, Router};

use crate::{
    repo::Repo,
    service::{errors::AppError, tag::ListTagReply},
};

pub fn tag_router(repo: Repo) -> Router {
    Router::new().route("/", get(list_tags)).with_state(repo)
}

pub async fn list_tags(state: State<Repo>) -> anyhow::Result<Json<ListTagReply>, AppError> {
    let tags = state.list_tags().await?;
    Ok(Json(ListTagReply { data: tags }))
}
