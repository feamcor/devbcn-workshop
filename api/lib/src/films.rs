use actix_web::{HttpResponse, web::{self, ServiceConfig}};

pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn service(configuration: &mut ServiceConfig) {
    configuration.service(
        web::scope("/v1/films")
        .route("", web::get().to(get_all))
        .route("/{film_id}", web::get().to(get))
        .route("", web::post().to(post))
        .route("", web::put().to(put))
        .route("/{film_id}", web::delete().to(delete)),
    );
}
    
async fn get_all() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn put() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}