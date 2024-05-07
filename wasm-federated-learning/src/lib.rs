mod client;
mod server;
mod utils;

use crate::client::clientutils::clientadd;
use crate::client::sql::option::{create, Book};
use crate::server::serverutils::serveradd;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let a = clientadd(30, 34).to_string();
    let b = serveradd(32, 34).to_string();
    let message = a + &b;
    let _ = sqloption();
    alert(&message)
}

// test for sql operation with wasm
#[tokio::main]
async fn sqloption() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let pool = sqlx::postgres::PgPool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    //create
    let book = Book {
        title: "Yang".to_string(),
        auther: "Xiang".to_string(),
        isbn: "BBBB".to_string(),
    };
    let _ = create(&book, &pool).await;

    Ok(())
}
