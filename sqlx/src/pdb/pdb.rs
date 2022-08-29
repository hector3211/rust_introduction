use dotenv::dotenv;
use sqlx::postgres::PgPool;
use sqlx::{query_as, FromRow};
use std::env;
#[derive(sqlx::FromRow, Debug)]
pub struct Product {
    pub id: i32,
    #[sqlx(rename = "porduct_name")]
    pub porduct_name: String,
    pub product_price: i32,
}

pub async fn db_connection() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    Ok(())
}

pub async fn add_product(pool: &PgPool, product: Product) -> anyhow::Result<Product> {
    let new_product = query_as!(
        Product,
        r#"
        INSERT INTO store (id,porduct_name,product_price)
        VALUES ($1,$2,$3)
        RETURNING id,porduct_name,product_price
        "#,
        &product.id,
        &product.porduct_name,
        &product.product_price,
    )
    .fetch_one(&pool.clone())
    .await?;
    Ok(Product {
        id: new_product.id,
        porduct_name: new_product.porduct_name,
        product_price: new_product.product_price,
    })
}
