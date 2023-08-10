use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use sqlx::Pool;
use api_lib::health::*;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: Pool<sqlx::Postgres>,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    tracing::info!("Initializing the database if not already done");
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    tracing::info!("Wrapping database pool on application data");
    let pool = actix_web::web::Data::new(pool);
    tracing::info!("Registering application configuration");
    let config = move |conf: &mut ServiceConfig| {
        conf.app_data(pool)
            .service(hello_world)
            .service(version);
    };

    Ok(config.into())
}
