use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use surrealdb_helper::SurrealDB;
use validator::Validate;

use crate::{
    models::{dto::Advertiser, ApiError as ApiErrorStruct},
    routes::ApiError,
    utils::validation::parse_validation_errors,
};

#[utoipa::path(
    tag = "Advertisers",
    operation_id = "upsert_advertisers",
    summary = "Массовое создание/обновление рекламодателей",
    description = "Создаёт новых или обновляет существующих рекламодателей",
    responses(
        (status = 201, description = "Успешное создание/обновление рекламодателей", body = Vec<Advertiser>),
        (status = 400, description = "Объект рекламодателя не соответствует модели", body = ApiErrorStruct)
    ),
)]
#[post("/bulk")]
pub async fn post_handler(
    db: Data<SurrealDB>,
    Json(body): Json<Vec<Advertiser>>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(parse_validation_errors)?;
    Ok(HttpResponse::Created().json(Advertiser::upsert(body, &db).await?))
}
