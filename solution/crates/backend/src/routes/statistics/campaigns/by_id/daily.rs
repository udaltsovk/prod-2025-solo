use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;

use crate::{
    models::{dto::Stats, url::CampaignIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

#[utoipa::path(
    tag = "Statistics",
    operation_id = "get_campaign_daily_stats",
    summary = "Получение ежедневной статистики по рекламной кампании",
    description = "Возвращает массив ежедневной статистики для указанной рекламной кампании.",
    params(
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, для которой запрашивается ежедневная статистика."),
    ),
    responses(
        (status = 200, description = "Ежедневная статистика по рекламной кампании успешно получена.", body = Stats),
        (status = 404, description = "Рекламной кампании с указанным UUID не существует.", body = ApiErrorStruct)

    ),
)]
#[get("/daily")]
pub async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<CampaignIdPath>,
) -> Result<Json<Vec<Stats>>, ApiError> {
    Ok(Json(Stats::campaign_daily(path.campaign_id, &db).await?))
}
