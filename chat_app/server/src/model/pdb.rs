use sqlx::postgres::PgPool;
use std::Fs;

pub async fn data_write_to_db(db: PgPool, file: &str) -> Result<(), sqlx::Error> {
    let conent = fs::read_to_string(file).map_err(|er| {
        println!("Error Reading {} reason:{}", file, er);
        er
    })?;
    let sqls: Vec<&str> = content.split(";").collect();
    for sql in sqls {
        match slqx::query(&sql).execute(db).await {
            Ok(_) => (),
            Err(er) => println!("WARNING in {} reason:{}", file, er),
        }
    }
    Ok(())
}

pub async fn db_connection() -> Result<(), sqlx::Error> {
    let db_url: &str = "postgres://localhost/test3";
    let pool = PgPool::connect(&db_url).await?;
    Ok(())
}
