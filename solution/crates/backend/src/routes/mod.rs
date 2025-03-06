use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use utoipa_actix_web::service_config::ServiceConfig;

mod ads;
mod advertisers;
mod clients;
mod ml_scores;
mod not_found;
mod statistics;
mod time;

use crate::models::ApiError as ApiErrorStruct;

pub use self::not_found::not_found;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.configure(clients::config)
        .configure(advertisers::config)
        .service(ml_scores::post_handler)
        .configure(ads::config)
        .configure(statistics::config)
        .configure(time::config);
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] surrealdb::Error),

    #[error("File host error: {0}")]
    FileHost(#[from] minio::s3::error::Error),

    #[error("{0} was not found")]
    NotFound(String),

    #[error("You're not allowed to do this")]
    NotOwner,

    #[error("Deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Error while validating input: {0}")]
    Validation(String),

    #[error("Unable to change field `{0}`: Campaign has already started")]
    CampaignStarted(String),

    #[error("{message}")]
    Custom {
        error: String,
        status_code: StatusCode,
        message: String,
    },
}

impl ApiError {
    pub fn as_api_error(&self) -> ApiErrorStruct {
        ApiErrorStruct {
            error: match self {
                Self::Database(..) => "database_error",
                Self::FileHost(..) => "file_host_error",
                Self::NotFound(..) => "not_found",
                Self::NotOwner => "not_owner",
                Self::Json(..) => "json_error",
                Self::InvalidInput(..) => "invalid_input",
                Self::Validation(..) => "invalid_input",
                Self::CampaignStarted(..) => "campaign_started",
                Self::Custom { error, .. } => error,
            }
            .to_string(),
            description: self.to_string(),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::FileHost(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(..) => StatusCode::NOT_FOUND,
            Self::NotOwner => StatusCode::FORBIDDEN,
            Self::Json(..) => StatusCode::BAD_REQUEST,
            Self::InvalidInput(..) => StatusCode::BAD_REQUEST,
            Self::Validation(..) => StatusCode::BAD_REQUEST,
            Self::CampaignStarted(..) => StatusCode::CONFLICT,
            Self::Custom { status_code, .. } => *status_code,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.as_api_error())
    }
}
