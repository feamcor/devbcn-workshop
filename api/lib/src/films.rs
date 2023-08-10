use actix_web::{HttpResponse, web::{self, ServiceConfig}};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;
use crate::film_repository::FilmRepository;

pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn service<R: FilmRepository>(configuration: &mut ServiceConfig) {
    configuration.service(
        web::scope("/v1/films")
        .route("", web::get().to(get_all::<R>))
        .route("/{film_id}", web::get().to(get::<R>))
        .route("", web::post().to(post::<R>))
        .route("", web::put().to(put::<R>))
        .route("/{film_id}", web::delete().to(delete::<R>)),
    );
}
    
async fn get_all<R: FilmRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)), 
    }
}

async fn get<R: FilmRepository>(film_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

async fn post<R: FilmRepository>(film: web::Json<CreateFilm>, repo: web::Data<R>) -> HttpResponse {
    match repo.create_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn put<R: FilmRepository>(film: web::Json<Film>, repo: web::Data<R>) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete<R: FilmRepository>(film_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}