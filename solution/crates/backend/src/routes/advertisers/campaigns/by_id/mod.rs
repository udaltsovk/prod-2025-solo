use actix_web::{
    delete, get, put,
    web::{Data, Json},
    HttpResponse,
};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;
use utoipa_actix_web::{scope, service_config::ServiceConfig};
use validator::Validate;

use crate::{
    models::{
        dto::{Campaign, CampaignUpdate},
        url::AdvertiserIdCampaignIdPath,
        ApiError as ApiErrorStruct,
    },
    routes::ApiError,
    utils::validation::parse_validation_errors,
};

mod image;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{campaign_id}")
            .service(get_handler)
            .service(put_handler)
            .service(delete_handler)
            .configure(image::config),
    );
}

#[utoipa::path(
    tag = "Campaigns",
    operation_id = "get_campaign_by_id",
    summary = "Получение кампании по ID",
    description = "Возвращает информацию о кампании по её ID.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, которому принадлежит кампания."),
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, которую необходимо получить.")
    ),
    responses(
        (status = 200, description = "Кампания успешно получена.", body = Campaign),
        (status = 404, description = "Рекламодателя или рекламной кампании с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[get("")]
async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdCampaignIdPath>,
) -> Result<Json<Campaign>, ApiError> {
    Ok(Json(
        Campaign::get_by_id(path.advertiser_id, path.campaign_id, &db).await?,
    ))
}

#[utoipa::path(
    tag = "Campaigns",
    operation_id = "update_campaign",
    summary = "Обновление рекламной кампании",
    description = "Обновляет разрешённые параметры рекламной кампании до её старта.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, которому принадлежит кампания."),
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, которую необходимо обновить.")
    ),
    request_body(description = "Объект с обновлёнными данными рекламной кампании.", content = CampaignUpdate),
    responses(
        (status = 204, description = "Рекламная кампания успешно обновлена.", body = Campaign),
        (status = 400, description = "Объект рекламной кампании не соответствует модели", body = ApiErrorStruct),
        (status = 404, description = "Рекламодателя или рекламной кампании с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[put("")]
async fn put_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdCampaignIdPath>,
    Json(body): Json<CampaignUpdate>,
) -> Result<Json<Campaign>, ApiError> {
    body.validate().map_err(parse_validation_errors)?;
    Ok(Json(
        Campaign::update(path.advertiser_id, path.campaign_id, body, &db).await?,
    ))
}

#[utoipa::path(
    tag = "Campaigns",
    operation_id = "delete_campaign",
    summary = "Удаление рекламной кампании",
    description = "Удаляет рекламную кампанию рекламодателя по заданному campaign_id.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, которому принадлежит кампания."),
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, которую необходимо удалить.")
    ),
    responses(
        (status = 204, description = "Рекламная кампания успешно удалена.", body = Campaign),
        (status = 404, description = "Рекламодателя или рекламной кампании с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[delete("")]
async fn delete_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdCampaignIdPath>,
) -> Result<HttpResponse, ApiError> {
    Campaign::delete(path.advertiser_id, path.campaign_id, &db).await?;
    Ok(HttpResponse::NoContent().into())
}
