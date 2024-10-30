use std::env;

use dotenv::dotenv;
use family_estate_server::{repo::Repo, server::Server};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let repo = Repo::new(&database_url()?).await?;
    let app = Server::new(repo);
    app.run(port()?).await?;
    Ok(())
}

fn port() -> anyhow::Result<usize> {
    let port = env::var("SERVER_PORT")?;
    Ok(port.parse::<usize>()?)
}

fn database_url() -> anyhow::Result<String> {
    Ok(env::var("DATABASE_URL")?)
}
