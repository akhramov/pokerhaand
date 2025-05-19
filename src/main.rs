mod options;

use pokerhaand::router;
use sqlx::SqlitePool;

#[tokio::main(flavor = "current_thread")]
async fn main() -> eyre::Result<()> {
    let options = options::from_env()?;
    let pool = SqlitePool::connect(&options.database_url).await?;

    sqlx::migrate!().run(&pool).await?;

    let app = router(Default::default(), pool);
    let listener = tokio::net::TcpListener::bind(options.address).await?;
    axum::serve(listener, app).await.map_err(Into::into)
}
