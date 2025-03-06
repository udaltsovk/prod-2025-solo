use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    models::{dto::Stats, url::CampaignIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

mod daily;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{campaign_id}")
            .service(get_handler)
            .service(daily::get_handler),
    );
}

#[utoipa::path(
    tag = "Statistics",
    operation_id = "get_campaign_stats",
    summary = "Получение статистики по рекламной кампании",
    description = "Возвращает агрегированную статистику (показы, переходы, затраты и конверсию) для заданной рекламной кампании.",
    params(
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, для которой запрашивается статистика."),
    ),
    responses(
        (status = 200, description = "Статистика по рекламной кампании успешно получена.", body = Stats),
        (status = 404, description = "Рекламной кампании с указанным UUID не существует.", body = ApiErrorStruct)

    ),
)]
#[get("")]
async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<CampaignIdPath>,
) -> Result<Json<Stats>, ApiError> {
    Ok(Json(Stats::campaign(path.campaign_id, &db).await?))
}
