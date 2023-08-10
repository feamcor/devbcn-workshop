use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use sqlx::Pool;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: Pool<sqlx::Postgres>,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    tracing::info!("Initializing the database if not already done");
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository = actix_web::web::Data::new(film_repository);
    let configuration = move |c: &mut ServiceConfig| {
        c.app_data(film_repository)
         .configure(api_lib::health::service)
         .configure(api_lib::films::service);
    };

    Ok(configuration.into())
}
