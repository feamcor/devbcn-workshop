use actix_web::{HttpResponse, web::{self, ServiceConfig}};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;
use crate::film_repository::FilmRepository;

type Repository = web::Data<Box<dyn FilmRepository>>;

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
    
async fn get_all(repo: Repository) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)), 
    }
}

async fn get(film_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

async fn post(film: web::Json<CreateFilm>, repo: Repository) -> HttpResponse {
    match repo.create_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn put(film: web::Json<Film>, repo: Repository) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete(film_id: web::Path<Uuid>, repo: Repository) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}