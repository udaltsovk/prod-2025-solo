use actix_web::{
    post,
    web::{Data, Json},
};
use surrealdb_helper::SurrealDB;
use validator::Validate;

use crate::{
    models::{dto::Time, ApiError as ApiErrorStruct},
    routes::ApiError,
    utils::validation::parse_validation_errors,
};

#[utoipa::path(
    tag = "Time",
    operation_id = "advance_day",
    summary = "Установка текущей даты",
    description = "Устанавливает текущий день в системе в заданную дату.",
    responses(
        (status = 200, description = "Текущая дата обновлена", body = Time),
        (status = 400, description = "Новая дата раньше текущей", body = ApiErrorStruct)
    ),
)]
#[post("/advance")]
pub async fn post_handler(
    db: Data<SurrealDB>,
    Json(body): Json<Time>,
) -> Result<Json<Time>, ApiError> {
    body.validate().map_err(parse_validation_errors)?;
    Ok(Json(Time::advance_day(body, &db).await?))
}
