use crate::models::ApiError;
use actix_web::{HttpResponse, Responder};

pub async fn not_found() -> impl Responder {
    let data = ApiError {
        error: "not_found".into(),
        description: "the requested route does not exist".into(),
    };

    HttpResponse::NotFound().json(data)
}
