use crate::database::book::book_create;
use crate::database::{book, user};
use crate::errors::MyError;
use crate::model::book::Book;
use actix_web::{web, Error, HttpResponse, Result};

pub async fn get_all_project(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    //create book
    let book = Book {
        title: "Samlone00".to_string(),
        auther: "Xiang00".to_string(),
        isbn: "FFFF00".to_string(),
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
    ctx.insert("error", "My error");
    ctx.insert("project", "Project");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handler_register_project(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("Project", "");
    let s = tmpl
        .render("register_project.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
