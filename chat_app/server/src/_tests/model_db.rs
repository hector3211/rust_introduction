use super::db_connection;

#[tokio::test]
async fn model_db_connection() -> Result<(), Box<dyn std::error::Error>> {
    let db = db_connection().await?;
    Ok(())
}
