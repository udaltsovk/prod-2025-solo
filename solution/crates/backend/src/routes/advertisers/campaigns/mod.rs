use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse,
};
use actix_web_lab::extract::{Path, Query};
use surrealdb_helper::SurrealDB;
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

use crate::{
    models::{
        dto::{Campaign, CreateCampaign},
        url::{AdvertiserIdPath, Pagination},
        ApiError as ApiErrorStruct,
    },
    routes::ApiError,
    utils::validation::parse_validation_errors,
};

mod by_id;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{advertiser_id}/campaigns")
            .service(post_handler)
            .service(get_handler)
            .configure(by_id::config),
    );
}

#[utoipa::path(
    tag = "Campaigns",
    operation_id = "create_campaign",
    summary = "Создание рекламной кампании",
    description = "Создаёт новую рекламную кампанию для указанного рекламодателя.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, для которого создаётся кампания.")
    ),
    request_body(description = "Объект с данными для создания рекламной кампании.", content = CreateCampaign),
    responses(
        (status = 201, description = "Рекламная кампания успешно создана.", body = Campaign),
        (status = 400, description = "Объект рекламной кампании не соответствует модели", body = ApiErrorStruct),
        (status = 404, description = "Рекламодателя с таким ID не существует.", body = ApiErrorStruct)

    ),
)]
#[post("")]
async fn post_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdPath>,
    Json(body): Json<CreateCampaign>,
) -> Result<HttpResponse, ApiError> {
    body.validate().map_err(parse_validation_errors)?;
    Ok(HttpResponse::Created().json(Campaign::create(path.advertiser_id, body, &db).await?))
}

#[utoipa::path(
    tag = "Campaigns",
    operation_id = "list_campaigns",
    summary = "Получение рекламных кампаний рекламодателя c пагинацией",
    description = "Возвращает список рекламных кампаний для указанного рекламодателя с пагинацией.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, для которого запрашиваются кампании."),
        Pagination
    ),
    responses(
        (status = 200, description = "Список рекламных кампаний рекламодателя.", body = Vec<Campaign>),
        (status = 400, description = "Некорректные параметры пагинации.", body = ApiErrorStruct),
        (status = 404, description = "Рекламодателя с таким ID не существует.", body = ApiErrorStruct)

    ),
)]
#[get("")]
async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdPath>,
    Query(query): Query<Pagination>,
) -> Result<Json<Vec<Campaign>>, ApiError> {
    query.validate().map_err(parse_validation_errors)?;
    Ok(Json(Campaign::list(path.advertiser_id, query, &db).await?))
}
