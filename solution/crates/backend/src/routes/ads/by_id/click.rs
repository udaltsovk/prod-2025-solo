use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use actix_web_lab::extract::Path;
use serde::Deserialize;
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    models::{dto::Ad, url::AdIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

#[derive(Deserialize, ToSchema, Debug)]
struct ClickRequest {
    /// UUID клиента, совершившего клик по объявлению.
    client_id: Uuid,
}

#[utoipa::path(
    tag = "Ads",
    operation_id = "record_ad_click",
    summary = "Фиксация перехода по рекламному объявлению",
    description = "Фиксирует клик (переход) клиента по рекламному объявлению.",
    params(
        ("ad_id" = Uuid, description = "UUID рекламного объявления (идентификатор кампании), по которому совершен клик.")
    ),
    responses(
        (status = 204, description = "Переход по рекламному объявлению успешно зафиксирован."),
        (status = 409, description = "Клиент не видел данное рекламное объявление", body = ApiErrorStruct),
        (status = 404, description = "Клиента или рекламного объявления с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[post("/click")]
pub async fn post_handler(
    db: Data<SurrealDB>,
    Path(path): Path<AdIdPath>,
    Json(body): Json<ClickRequest>,
) -> Result<HttpResponse, ApiError> {
    Ad::record_click(body.client_id, path.ad_id, &db).await?;
    Ok(HttpResponse::NoContent().into())
}
