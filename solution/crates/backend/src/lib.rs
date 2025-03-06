use actix_web::web::{get, Data, JsonConfig, PathConfig};
use env_config::environment_variables;
use surrealdb_helper::SurrealDB;
use utils::{minio::Minio, openapi::Swagger};
use utoipa::openapi::OpenApi as OpenApiStruct;
use utoipa::OpenApi;
use utoipa_actix_web::service_config::ServiceConfig;

pub mod models;
pub mod routes;
pub mod utils;

use crate::routes::{not_found, ApiError};

environment_variables! {
    SERVER_ADDRESS: String = "0.0.0.0:8080",
    DB_ADDRESS: String = "localhost:8001",
    DB_NAMESPACE: String = "ad_platform",
    DB_NAME: String = "backend",
    DB_USER: String = "root",
    DB_PASSWORD: String = "root",
    MINIO_BASE_URL: String = "http://localhost:9000",
    MINIO_USER: String = "root",
    MINIO_PASSWORD: String = "beetroot",
    MINIO_BUCKET: String = "ad-platform-backend-bucket",
    MODERATION_ENABLED: bool = true
}

pub fn app_setup(db: SurrealDB, minio: Minio) -> BackendConfig {
    config::init();
    BackendConfig {
        db,
        minio,
        openapi: Swagger::openapi(),
    }
}

#[derive(Clone)]
pub struct BackendConfig {
    pub db: SurrealDB,
    pub minio: Minio,
    pub openapi: OpenApiStruct,
}

impl BackendConfig {
    pub fn build(self) -> impl FnOnce(&mut ServiceConfig) {
        move |cfg: &mut ServiceConfig| {
            cfg.app_data(
                PathConfig::default()
                    .error_handler(|err, _req| ApiError::Validation(err.to_string()).into()),
            )
            .app_data(
                JsonConfig::default()
                    .error_handler(|err, _req| ApiError::Validation(err.to_string()).into()),
            )
            .app_data(Data::new(self.db))
            .app_data(Data::new(self.minio))
            .configure(routes::config)
            .default_service(get().to(not_found));
        }
    }
}
