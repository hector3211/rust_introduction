use sqlx::postgres::PgPool;
#[allow(unused)]
fn main() {
    pub async fn db_connection() -> Result<(), sqlx::Error> {
        let db_url: &str = "postgres://localhost/test3";
        let pool = PgPool::connect(&db_url).await?;
        Ok(())
    }
    db_connection();
}
