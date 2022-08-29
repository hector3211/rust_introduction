mod pdb;
use pdb::pdb::db_connection;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    db_connection().await?;
    Ok(())
}
