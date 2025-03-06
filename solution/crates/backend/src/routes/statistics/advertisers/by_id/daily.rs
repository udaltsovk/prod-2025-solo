use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;

use crate::{
    models::{dto::Stats, url::AdvertiserIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

#[utoipa::path(
    tag = "Statistics",
    operation_id = "get_advertiser_daily_stats",
    summary = "Получение ежедневной агрегированной статистики по всем кампаниям рекламодателя",
    description = "Возвращает массив ежедневной сводной статистики по всем рекламным кампаниям заданного рекламодателя.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, для которого запрашивается ежедневная статистика по кампаниям."),
    ),
    responses(
        (status = 200, description = "Ежедневная агрегированная статистика успешно получена.", body = Stats),
        (status = 404, description = "Рекламодателя с указанным UUID не существует.", body = ApiErrorStruct)

    ),
)]
#[get("/daily")]
pub async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdPath>,
) -> Result<Json<Vec<Stats>>, ApiError> {
    Ok(Json(
        Stats::advertiser_daily(path.advertiser_id, &db).await?,
    ))
}
