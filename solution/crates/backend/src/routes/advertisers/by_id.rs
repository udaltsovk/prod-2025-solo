use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;

use crate::{
    models::{dto::Advertiser, url::AdvertiserIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

#[utoipa::path(
    tag = "Advertisers",
    operation_id = "get_advertiser_by_id",
    summary = "Получение рекламодателя по ID",
    description = "Возвращает информацию о рекламодателе по его ID.",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя.")
    ),
    responses(
        (status = 200, description = "Информация о рекламодателе успешно получена.", body = Advertiser),
        (status = 404, description = "Рекламодателя с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[get("/{advertiser_id}")]
pub async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdvertiserIdPath>,
) -> Result<Json<Advertiser>, ApiError> {
    Ok(Json(Advertiser::get_by_id(path.advertiser_id, &db).await?))
}
