use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Book {
    pub title: String,
    pub auther: String,
    pub isbn: String,
}
