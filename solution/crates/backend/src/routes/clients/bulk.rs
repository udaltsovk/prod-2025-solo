use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use surrealdb_helper::SurrealDB;
use validator::Validate;

use crate::{
    models::{dto::Client, ApiError as ApiErrorStruct},
    routes::ApiError,
    utils::validation::parse_validation_errors,
};

#[utoipa::path(
    tag = "Clients",
    operation_id = "upsert_clients",
    summary = "Массовое создание/обновление клиентов",
    description = "Создаёт новых или обновляет существующих клиентов",
    responses(
        (status = 201, description = "Успешное создание/обновление клиентов", body = Vec<Client>),
        (status = 400, description = "Объект клиента не соответствует модели", body = ApiErrorStruct)
    ),
)]
#[post("/bulk")]
pub async fn post_handler(
    db: Data<SurrealDB>,
    Json(body): Json<Vec<Client>>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(parse_validation_errors)?;
    Ok(HttpResponse::Created().json(Client::upsert(body, &db).await?))
}
