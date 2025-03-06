use actix_web::{get, web::Data, HttpResponse};
use actix_web_lab::extract::Query;
use surrealdb_helper::SurrealDB;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    models::{dto::Ad, url::ClientIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

mod by_id;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/ads").service(get_handler).configure(by_id::config));
}

#[utoipa::path(
    tag = "Ads",
    operation_id = "get_ad_for_client",
    summary = "Получение рекламного объявления для клиента",
    description = "Возвращает рекламное объявление, подходящее для показа клиенту с учетом таргетинга и ML скора.",
    params(
        ("client_id" = Uuid, Query, description = "UUID клиента, запрашивающего показ объявления."),
    ),
    responses(
        (status = 200, description = "Рекламное объявление успешно возвращено.", body = Ad),
        (status = 204, description = "Не удалось найти подходящее рекламное объявление."),
        (status = 404, description = "Клиента с указанным UUID не существует.", body = ApiErrorStruct)

    ),
)]
#[get("")]
async fn get_handler(
    db: Data<SurrealDB>,
    Query(query): Query<ClientIdPath>,
) -> Result<HttpResponse, ApiError> {
    Ok(Ad::get_for_client(query.client_id, &db).await?)
}
