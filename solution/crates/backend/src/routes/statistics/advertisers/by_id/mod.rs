use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    models::{dto::Stats, url::AdvertiserIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

mod daily;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/{advertiser_id}")
            .service(get_handler)
            .service(daily::get_handler),
    );
}

#[utoipa::path(
    tag = "Statistics",
    operation_id = "get_advertiser_campaigns_stats",
    summary = "Получение агрегированной статистики по всем кампаниям рекламодателя",
    description = "Возвращает сводную статистику по всем рекламным кампаниям, принадлежащим заданному рекламодателю.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, для которого запрашивается статистика."),
    ),
    responses(
        (status = 200, description = "Агрегированная статистика по всем кампаниям рекламодателя успешно получена.", body = Stats),
        (status = 404, description = "Рекламодателя с указанным UUID не существует.", body = ApiErrorStruct)

    ),
)]
#[get("")]
async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdPath>,
) -> Result<Json<Stats>, ApiError> {
    Ok(Json(Stats::advertiser(path.advertiser_id, &db).await?))
}
