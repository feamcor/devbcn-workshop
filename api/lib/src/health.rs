use actix_web::get;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting database version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    tracing::info!("Handling database version result");
    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}