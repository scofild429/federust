pub mod option {
    use std::error::Error;

    use futures::TryStreamExt;
    use serde::{Deserialize, Serialize};
    use sqlx::{prelude::FromRow, Row};

    #[derive(Debug, FromRow)]
    pub struct Book {
        pub title: String,
        pub auther: String,
        pub isbn: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Metadata {
        pub avg_review: f32,
        pub tags: Vec<String>,
    }

    pub async fn delete(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "DELETE FROM book WHERE isbn = ($1)";
        sqlx::query(query).bind(&book.isbn).execute(pool).await?;
        Ok(())
    }

    pub async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO book (title, auther, isbn) VALUES ($1, $2, $3)";
        sqlx::query(query)
            .bind(&book.title)
            .bind(&book.auther)
            .bind(&book.isbn)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "UPDATE book SET title = $1, auther = $2 WHERE isbn = $3";
        sqlx::query(query)
            .bind(&book.title)
            .bind(&book.auther)
            .bind(&book.isbn)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn read_one(conn: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
        let q = "SELECT title, auther, isbn FROM book";
        let query = sqlx::query(q);
        let row = query.fetch_one(conn).await?;
        let book = Book {
            title: row.get("title"),
            auther: row.get("auther"),
            isbn: row.get("isbn"),
        };
        Ok(book)
    }

    pub async fn read_option(conn: &sqlx::PgPool) -> Result<Option<Book>, Box<dyn Error>> {
        let q = "SELECT title, auther, isbn FROM book";
        let query = sqlx::query(q);
        let maybe_row = query.fetch_optional(conn).await?;
        let book = maybe_row.map(|row| Book {
            title: row.get("title"),
            auther: row.get("auther"),
            isbn: row.get("isbn"),
        });

        Ok(book)
    }

    pub async fn read_all(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
        let q = "SELECT title, auther, isbn FROM book";
        let query = sqlx::query(q);
        let rows = query.fetch_all(conn).await?;
        let books = rows
            .iter()
            .map(|row| Book {
                title: row.get("title"),
                auther: row.get("auther"),
                isbn: row.get("isbn"),
            })
            .collect();
        Ok(books)
    }

    pub async fn read(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
        let q = "SELECT title, auther, isbn FROM book";
        let query = sqlx::query(q);
        let mut rows = query.fetch(conn);
        let mut books = vec![];

        while let Some(row) = rows.try_next().await? {
            books.push(Book {
                title: row.get("title"),
                auther: row.get("auther"),
                isbn: row.get("isbn"),
            })
        }
        Ok(books)
    }

    pub async fn query_as(conn: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
        let q = "SELECT title, auther, isbn FROM book";
        let query = sqlx::query_as::<_, Book>(q);
        let books = query.fetch_all(conn).await?;
        Ok(books)
    }
}
