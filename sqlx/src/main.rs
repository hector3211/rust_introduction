use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://localhost/test3")
        .await?;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ticket (
        id bigserial,
        name text
        );
        "#,
    )
    .execute(&pool)
    .await?;
    Ok(())
}
