use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Book {
    pub title: String,
    pub auther: String,
    pub isbn: String,
}
