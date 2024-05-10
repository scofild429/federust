use crate::database::book::{create, delete, query_as, read_all, read_one, read_option};
use crate::model::book::Book;
use dotenv::dotenv;
use sqlx;
use std::env;
use std::error::Error;
use tokio;

// #[tokio::main]
pub async fn option() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let pool = sqlx::postgres::PgPool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    //create book
    let book = Book {
        title: "Samlone".to_string(),
        auther: "Xiang".to_string(),
        isbn: "FFFFe".to_string(),
    };
    let _ = create(&book, &pool).await;

    //................................................................

    //delete
    // let book = Book {
    //     title: ""Samlone".to_string(),
    //     auther: "Yang".to_string(),
    //     isbn: "AAAA".to_string(),
    // };
    // let _ = delete(&book, &pool).await;
    //................................................................

    //update
    // let book0 = Book {
    //     title: "Samlone".to_string(),
    //     auther: "Yang".to_string(),
    //     isbn: "AAAA".to_string(),
    // };
    // let _ = update(&book0, &pool).await;
    //................................................................

    //read
    // let books = read(&pool).await?;
    // println!("{:?}", books);
    //................................................................

    //read_one
    // let books = read_one(&pool).await?;
    // println!("{:?}", books);
    //................................................................

    //read_option
    // let books = read_option(&pool).await?;
    // println!("{:?}", books);
    //................................................................

    //read_all
    // let books = read_all(&pool).await?;
    // println!("{:?}", books);
    //................................................................

    //query_as
    let books = query_as(&pool).await?;
    println!("{:?}", books);
    //................................................................

    Ok(())
}
