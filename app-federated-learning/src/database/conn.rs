use actix_web::Error as Actix_Error;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;

pub async fn database_connection() -> Result<sqlx::PgPool, Actix_Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Host is not set!");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    Ok(pool)
}

pub async fn conn() -> Result<sqlx::PgPool, Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Host is not set!");
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    Ok(pool)
}
