use actix_web::{get, HttpResponse};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", env!("CARGO_PKG_VERSION")))
        .finish()
}