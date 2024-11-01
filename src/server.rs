use crate::{
    repo::Repo,
    router::{category::category_router, tag::tag_router},
    usecase::{category::CategoryUsecase, tag::TagUsecase},
};
use axum::Router;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(repo: Repo) -> Self {
        let router = Router::new()
            .nest("/tag", tag_router(TagUsecase::new(&repo)))
            .nest("/category", category_router(CategoryUsecase::new(&repo)));
        Server { router }
    }
    pub async fn run(self, port: usize) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
