use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use sqlx::Pool;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}


#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: Pool<sqlx::Postgres>,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let pool = actix_web::web::Data::new(pool);
    let config = move |conf: &mut ServiceConfig| {
        conf.app_data(pool)
            .service(hello_world)
            .service(version);
    };

    Ok(config.into())
}
