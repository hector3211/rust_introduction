use sqlx::postgres::PgPool;
use std::env;

pub async fn establish_connection() -> anyhow::Result<()> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    // create a todo table if not exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todo (
        id bigserial,
        name text
        );"#,
    )
    .execute(&pool)
    .await?;

    Ok(())
}
