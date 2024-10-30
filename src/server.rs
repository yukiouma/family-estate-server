use crate::repo::Repo;
use axum::Router;

pub struct Server {
    router: Router,
}

impl Server {
    pub fn new(repo: Repo) -> Self {
        let router = Router::new().with_state(repo);
        Server { router }
    }
    pub async fn run(self, port: usize) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
