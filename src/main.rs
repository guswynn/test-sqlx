use anyhow::Result;
use sqlx::postgres::PgPoolOptions;

fn main() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(10)
        .build()?;

    sqlx::rt::set_runtime(runtime.handle().clone())
        .map_err(|_| anyhow::anyhow!("lost the runtime race"))?;

    runtime.block_on(async move {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:password@localhost/test")
            .await?;

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&pool)
            .await?;

        Ok(())
    })
}
