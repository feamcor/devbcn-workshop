use actix_web::{HttpResponse, web::{self, ServiceConfig}};

pub const API_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn service(configuration: &mut ServiceConfig) {
    configuration.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let response = health().await;
        assert!(response.status().is_success());
        assert_eq!(response.status(), StatusCode::OK);
        let version = response.headers().get("version").and_then(|v| v.to_str().ok());
        assert_eq!(version, Some(API_VERSION));
    }
}