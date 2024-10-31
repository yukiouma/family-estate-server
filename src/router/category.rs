use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use category::category::Category;

use crate::{
    repo::Repo,
    service::{
        category::{
            CreateCategroyReply, CreateSubCategroyReply, CreateSubCategroyRequest,
            ListCategoryReply, ListSubCategoryRequest, ModifyCategroyReply, RemoveCategoryReply,
        },
        errors::AppError,
    },
};

pub fn category_router(repo: Repo) -> Router {
    Router::new()
        .route("/", get(list_category))
        .route("/sub", get(list_sub_category))
        .route("/", post(create_category))
        .route("/sub", post(create_sub_category))
        .route("/:id", put(modify_category))
        .route("/:id", delete(remove_category))
        .with_state(repo)
}

async fn list_category(state: State<Repo>) -> anyhow::Result<Json<ListCategoryReply>, AppError> {
    let categories = state.list_category().await?;
    Ok(Json(ListCategoryReply { data: categories }))
}

async fn list_sub_category(
    state: State<Repo>,
    query: Query<ListSubCategoryRequest>,
) -> anyhow::Result<Json<ListCategoryReply>, AppError> {
    let categories = state.list_sub_categories(query.category_id).await?;
    Ok(Json(ListCategoryReply { data: categories }))
}

async fn create_category(
    state: State<Repo>,
    Json(category): Json<Category>,
) -> anyhow::Result<Json<CreateCategroyReply>, AppError> {
    state.create_category(&category).await?;
    Ok(Json(CreateCategroyReply {}))
}

async fn create_sub_category(
    state: State<Repo>,
    Json(request): Json<CreateSubCategroyRequest>,
) -> anyhow::Result<Json<CreateSubCategroyReply>, AppError> {
    state
        .create_sub_category(
            request.parent,
            &Category {
                id: None,
                name: request.name.into(),
            },
        )
        .await?;
    Ok(Json(CreateSubCategroyReply {}))
}

async fn modify_category(
    state: State<Repo>,
    Path(id): Path<i64>,
    Json(category): Json<Category>,
) -> anyhow::Result<Json<ModifyCategroyReply>, AppError> {
    state
        .modify_category(&Category {
            id: Some(id),
            name: category.name.into(),
        })
        .await?;
    Ok(Json(ModifyCategroyReply {}))
}

async fn remove_category(
    state: State<Repo>,
    Path(id): Path<i64>,
) -> anyhow::Result<Json<RemoveCategoryReply>, AppError> {
    state.remove_category(id).await?;
    Ok(Json(RemoveCategoryReply {}))
}
