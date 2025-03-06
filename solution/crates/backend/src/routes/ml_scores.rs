use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use surrealdb_helper::SurrealDB;
use validator::Validate;

use crate::{
    models::{dto::MLScore, ApiError as ApiErrorStruct},
    routes::ApiError,
    utils::validation::parse_validation_errors,
};

#[utoipa::path(
    tag = "Advertisers",
    operation_id = "upsert_ml_score",
    summary = "Добавление или обновление ML скора",
    description = "Добавляет или обновляет ML скор для указанной пары клиент-рекламодатель.",
    request_body(
        description = "Объект с данными ML скора, включая client_id, advertiser_id и значение скора.", 
        content = MLScore
    ),
    responses(
        (status = 200, description = "ML скор успешно добавлен или обновлён."),
        (status = 404, description = "Рекламодателя или клиента с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[post("/ml-scores")]
pub async fn post_handler(
    db: Data<SurrealDB>,
    Json(body): Json<MLScore>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(parse_validation_errors)?;
    MLScore::upsert(body, &db).await?;
    Ok(HttpResponse::Ok().into())
}
