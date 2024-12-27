use crate::{
    repo::Repo,
    router::{
        category::category_router, data::data_router, statistic::statistic_router, tag::tag_router,
    },
    usecase::{
        category::CategoryUsecase, data::DataUsecase, statistic::StatisticUsecase, tag::TagUsecase,
    },
};
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(repo: Repo) -> Self {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        let router = Router::new()
            .nest(
                "/api",
                Router::new()
                    .nest("/tag", tag_router(TagUsecase::new(&repo)))
                    .nest("/category", category_router(CategoryUsecase::new(&repo)))
                    .nest("/data", data_router(DataUsecase::new(&repo)))
                    .nest("/statistic", statistic_router(StatisticUsecase::new(&repo))),
            )
            .layer(TraceLayer::new_for_http());
        Server { router }
    }
    pub async fn run(self, port: usize) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!("Family estate server listening port: {}", port);
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
