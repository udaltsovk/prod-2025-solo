use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;

use crate::{
    models::{dto::Client, url::ClientIdPath, ApiError as ApiErrorStruct},
    routes::ApiError,
};

#[utoipa::path(
    tag = "Clients",
    operation_id = "get_client_by_id",
    summary = "Получение клиента по ID",
    description = "Возвращает информацию о клиенте по его ID.",
    params(
        ("client_id" = Uuid, description = "UUID клиента.")
    ),
    responses(
        (status = 200, description = "Информация о клиенте успешно получена.", body = Client),
        (status = 404, description = "Клиента с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[get("/{client_id}")]
pub async fn get_handler(
    db: Data<SurrealDB>,
    Path(path): Path<ClientIdPath>,
) -> Result<Json<Client>, ApiError> {
    Ok(Json(Client::get_by_id(path.client_id, &db).await?))
}
