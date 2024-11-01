use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use tag::tag::Tag;

use crate::{
    service::{
        errors::AppError,
        tag::{CreateTagReply, ListTagReply, ModifyTagReply, RemoveTagReply},
    },
    usecase::tag::TagUsecase,
};

pub fn tag_router(usecase: TagUsecase) -> Router {
    Router::new()
        .route("/", get(list_tags))
        .route("/", post(create_tag))
        .route("/:id", put(modify_tag))
        .route("/:id", delete(remove_tag))
        .with_state(usecase)
}

async fn list_tags(state: State<TagUsecase>) -> anyhow::Result<Json<ListTagReply>, AppError> {
    let tags = state.list_tags().await?;
    Ok(Json(ListTagReply { data: tags }))
}

async fn create_tag(
    state: State<TagUsecase>,
    Json(tag): Json<Tag>,
) -> anyhow::Result<Json<CreateTagReply>, AppError> {
    state.create_tag(&tag).await?;
    Ok(Json(CreateTagReply {}))
}

async fn modify_tag(
    state: State<TagUsecase>,
    Path(id): Path<i64>,
    Json(tag): Json<Tag>,
) -> anyhow::Result<Json<ModifyTagReply>, AppError> {
    state
        .modify_tag(&Tag {
            id: Some(id),
            name: tag.name.into(),
        })
        .await?;
    Ok(Json(ModifyTagReply {}))
}

async fn remove_tag(
    state: State<TagUsecase>,
    Path(id): Path<i64>,
) -> anyhow::Result<Json<RemoveTagReply>, AppError> {
    state.remove_tag(id).await?;
    Ok(Json(RemoveTagReply {}))
}
