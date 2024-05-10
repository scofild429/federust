use crate::auth::register::{get_all_project, handler_register_project, register_project};

use actix_files as fs;

use actix_web::web;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(get_all_project)))
            .service(web::resource("/register").route(web::get().to(register_project)))
            .service(
                web::resource("/register-post").route(web::post().to(handler_register_project)),
            ),
    );
}
