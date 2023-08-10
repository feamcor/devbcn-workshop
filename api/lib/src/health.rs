use actix_web::{HttpResponse, web::{self, ServiceConfig}};

pub fn service(configuration: &mut ServiceConfig) {
    configuration.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", env!("CARGO_PKG_VERSION")))
        .finish()
}