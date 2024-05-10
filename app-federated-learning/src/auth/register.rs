use crate::client::models::ProjectRegisterForm;
use crate::database::book::book_create;
use crate::database::{book, user};
use crate::errors::MyError;
use crate::model::book::Book;
use actix_web::{web, Error, HttpResponse, Result};

pub async fn get_all_project(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    //create book
    let book = Book {
        title: "Samlone00".to_string(),
        auther: "Xiang22".to_string(),
        isbn: "FFFF011".to_string(),
    };

    let _ = book_create(&book).await;

    let mut ctx = tera::Context::new();
    ctx.insert("project", "my project 10");
    ctx.insert("error", "My error 20");
    let s = tmpl
        .render("project.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn register_project(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let book = Book {
        title: "Samlone00".to_string(),
        auther: "Xiang22".to_string(),
        isbn: "FFFF011".to_string(),
    };

    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("Book", &book);
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error at register".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handler_register_project(
    tmpl: web::Data<tera::Tera>,
    param: web::Form<ProjectRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let book = Book {
        title: "Samlone00".to_string(),
        auther: "Xiang22".to_string(),
        isbn: "FFFF011".to_string(),
    };
    if param.name == "DD" {
        ctx.insert("error", "DD not allowed!");
        ctx.insert("Book", &book);
        ctx.insert("current_name", &param.name);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error, DD not allowed".to_string()))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        s = format!("You are done");
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}
