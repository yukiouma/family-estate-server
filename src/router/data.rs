use crate::{
    service::{
        category::ModifyCategroyReply,
        data::{
            CreateDataReply, ListCategoryDataReply, ListCategoryDataRequest,
            ListCategoryHistoryReply, ListCategoryHistoryRequest, ListDataReply, ListDataRequest,
            ListHistoryReply, ListHistoryRequest, ModifyDataRequest, RemoveDataReply,
        },
        errors::AppError,
    },
    usecase::data::DataUsecase,
};
use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use data::data::Data;

pub fn data_router(usecase: DataUsecase) -> Router {
    Router::new()
        .route("/category", get(list_category_data))
        .route("/", get(list_data))
        .route("/history", get(list_history))
        .route("/category/history", get(list_category_history))
        .route("/", post(create_data))
        .route("/:id", put(modify_data))
        .route("/:id", delete(remove_data))
        .with_state(usecase)
}

async fn list_category_data(
    state: State<DataUsecase>,
    query: Query<ListCategoryDataRequest>,
) -> anyhow::Result<Json<ListCategoryDataReply>, AppError> {
    let data = state.list_category_data(query.tag_id).await?;
    Ok(Json(ListCategoryDataReply { data }))
}

async fn list_data(
    state: State<DataUsecase>,
    query: Query<ListDataRequest>,
) -> anyhow::Result<Json<ListDataReply>, AppError> {
    let data = state.list_data(query.tag_id).await?;
    Ok(Json(ListDataReply { data }))
}

async fn list_history(
    state: State<DataUsecase>,
    query: Query<ListHistoryRequest>,
) -> anyhow::Result<Json<ListHistoryReply>, AppError> {
    let data = state.list_history(query.record_id).await?;
    Ok(Json(ListHistoryReply { data }))
}

async fn list_category_history(
    state: State<DataUsecase>,
    query: Query<ListCategoryHistoryRequest>,
) -> anyhow::Result<Json<ListCategoryHistoryReply>, AppError> {
    let data = state.list_category_history(query.sub_category_id).await?;
    Ok(Json(ListCategoryHistoryReply { data }))
}

async fn create_data(
    state: State<DataUsecase>,
    Json(data): Json<Data>,
) -> anyhow::Result<Json<CreateDataReply>, AppError> {
    state.create_data(&data).await?;
    Ok(Json(CreateDataReply {}))
}

async fn modify_data(
    state: State<DataUsecase>,
    Path(id): Path<i64>,
    Json(request): Json<ModifyDataRequest>,
) -> anyhow::Result<Json<ModifyCategroyReply>, AppError> {
    state.modify_data(id, request.amount).await?;
    Ok(Json(ModifyCategroyReply {}))
}

async fn remove_data(
    state: State<DataUsecase>,
    Path(id): Path<i64>,
) -> anyhow::Result<Json<RemoveDataReply>, AppError> {
    state.remove_data(id).await?;
    Ok(Json(RemoveDataReply {}))
}
