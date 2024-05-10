mod auth;
mod client;
mod database;
mod errors;
mod model;
mod routers;
mod server;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;
use tera::Tera;

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    dotenv().ok();
    let host_post = env::var("HOST_PORT").expect("HOST_PORT address is not set!");
    println!("Listening on port: {}", &host_post);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind(&host_post)?
    .run()
    .await?;
    Ok(())
}

fn main() {
    println!("Starting the web application!");
    let _ = start();
}
