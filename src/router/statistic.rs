use crate::{
    service::{
        errors::AppError,
        statistic::{
            CreateStatisticReply, CreateStatisticRequest, GetStatisticReply, ListStatisticReply,
            ModifyStatisticReply, ModifyStatisticRequest, RemoveStatisticReply,
        },
    },
    usecase::statistic::StatisticUsecase,
};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use statistic::statistic::CreateStatistic;

pub fn statistic_router(usecase: StatisticUsecase) -> Router {
    Router::new()
        .route("/", get(list_statistic))
        .route("/:id", get(get_statistic_category))
        .route("/", post(create_statistic))
        .route("/:id", put(modify_statistic))
        .route("/:id", delete(remove_statistic))
        .with_state(usecase)
}

async fn list_statistic(
    state: State<StatisticUsecase>,
) -> anyhow::Result<Json<ListStatisticReply>, AppError> {
    let data = state.list_statistic().await?;
    Ok(Json(ListStatisticReply { data }))
}

async fn get_statistic_category(
    state: State<StatisticUsecase>,
    Path(id): Path<i64>,
) -> anyhow::Result<Json<GetStatisticReply>, AppError> {
    let data = state.get_statistic_category(id).await?;
    Ok(Json(GetStatisticReply { data }))
}

async fn create_statistic(
    state: State<StatisticUsecase>,
    Json(request): Json<CreateStatisticRequest>,
) -> anyhow::Result<Json<CreateStatisticReply>, AppError> {
    state
        .create_statistic(&CreateStatistic {
            name: request.name,
            categories: request.categories,
        })
        .await?;
    Ok(Json(CreateStatisticReply {}))
}

async fn modify_statistic(
    state: State<StatisticUsecase>,
    Path(id): Path<i64>,
    Json(request): Json<ModifyStatisticRequest>,
) -> anyhow::Result<Json<ModifyStatisticReply>, AppError> {
    state.modify_statistic(id, &request.name).await?;
    Ok(Json(ModifyStatisticReply {}))
}

async fn remove_statistic(
    state: State<StatisticUsecase>,
    Path(id): Path<i64>,
) -> anyhow::Result<Json<RemoveStatisticReply>, AppError> {
    state.remove_statistic(id).await?;
    Ok(Json(RemoveStatisticReply {}))
}
