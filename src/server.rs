use crate::{repo::Repo, router::tag::tag_router};
use axum::Router;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(repo: Repo) -> Self {
        let router = Router::new().nest("/tag", tag_router(repo.clone()));
        Server { router }
    }
    pub async fn run(self, port: usize) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
