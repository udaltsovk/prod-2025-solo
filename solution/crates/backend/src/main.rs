use actix_web::{
    middleware::{Compress, TrailingSlash},
    App, HttpServer,
};
use actix_web_lab::middleware::{CatchPanic, NormalizePath};
use backend::{
    app_setup, config,
    utils::{logger::CustomLogger, minio::Minio, openapi::Swagger},
};
use env_logger::Env;
use include_dir::include_dir;
use surrealdb_helper::SurrealDB;
use utoipa_actix_web::AppExt;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let default_log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    env_logger::init_from_env(Env::default().default_filter_or(default_log_level));

    let db = SurrealDB::init(
        &config::DB_ADDRESS,
        &config::DB_NAMESPACE,
        &config::DB_NAME,
        &config::DB_USER,
        &config::DB_PASSWORD,
    )
    .await
    .expect("Failed to init the database")
    .migrate(&include_dir!("crates/backend/db"))
    .await;

    let minio = Minio::init(
        &config::MINIO_BASE_URL,
        &config::MINIO_USER,
        &config::MINIO_PASSWORD,
        &config::MINIO_BUCKET,
    )
    .await
    .expect("Failed to init the file host");

    let app_config = app_setup(db, minio);

    HttpServer::new(move || {
        App::new()
            .wrap(CatchPanic::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(CustomLogger::new())
            .into_utoipa_app()
            .openapi(app_config.openapi.clone())
            .configure(app_config.clone().build())
            .openapi_service(Swagger::ui_service)
            .into_app()
    })
    .bind(config::SERVER_ADDRESS.clone())?
    .run()
    .await
}
