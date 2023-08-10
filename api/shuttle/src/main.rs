use actix_web::web;
use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;
use sqlx::Pool;
use api_lib::film_repository::PostgresFilmRepository;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: Pool<sqlx::Postgres>,
    #[shuttle_static_folder::StaticFolder(folder="static")] static_folder: std::path::PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    tracing::info!("Initializing the database if not already done");
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let film_repository = PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);
    let configuration = move |c: &mut ServiceConfig| {
        c.service(
            web::scope("/api")
                .app_data(film_repository)
                .configure(api_lib::health::service)
                .configure(api_lib::films::service::<PostgresFilmRepository>),
        ).service(
            actix_files::Files::new("/", static_folder)
                .show_files_listing()
                .index_file("index_html"),
        );
    };

    Ok(configuration.into())
}
