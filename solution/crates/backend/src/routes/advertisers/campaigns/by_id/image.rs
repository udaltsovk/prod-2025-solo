use crate::{models::{dto::Campaign, url::AdvertiserIdCampaignIdPath, ApiError as ApiErrorStruct}, routes::ApiError, utils::minio::Minio};
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{delete, get, http::StatusCode, put, web::Data, HttpResponse};
use actix_web_lab::extract::Path;
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/image")
            .service(put_handler)
            .service(get_handler)
            .service(delete_handler),
    );
}

#[derive(MultipartForm, ToSchema, Debug)]
struct UploadForm {
    #[schema(
        value_type = String, 
        format = Binary, 
        content_media_type = "application/octet-stream",
    )]
    /// Изображение, которое будет загружено. 
    /// Размер не должен превышать 5.7 МБ.
    /// Разрешённые MIME типы: `image/jpeg`, `image/pjpeg`, `image/png`, `image/webp`
    file: TempFile,
}

#[utoipa::path(
    tag = "Campaign images",
    operation_id = "set_campaign_image",
    summary = "Установка изображения рекламной кампании",
    description = "Добавляет изображение к рекламной кампании",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, которому принадлежит кампания."),
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, которой необходимо установить данное изображение.")
    ),
    request_body(content = UploadForm, content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Изображение успешно установлено."),
        (status = 415, description = "MIME тип изображения не разрешён", body = ApiErrorStruct),
        (status = 413, description = "Размер изображения превосходит максимально допустимый"),
        (status = 404, description = "Рекламодателя или рекламной кампании с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[put("")]
pub async fn put_handler(
    db: Data<SurrealDB>,
    minio: Data<Minio>,
    Path(path): Path<AdvertiserIdCampaignIdPath>,
    MultipartForm(form): MultipartForm<UploadForm>
) -> Result<HttpResponse, ApiError> {
    let AdvertiserIdCampaignIdPath { advertiser_id, campaign_id } = path;
    Campaign::get_by_id(advertiser_id, campaign_id, &db).await?;

    let file = form.file;

    if file.size > (5.7 * 1024 as f64 * 1024 as f64) as usize {
        return Err(ApiError::Custom { error: "file_too_large".into(), status_code: StatusCode::PAYLOAD_TOO_LARGE, message: "File size exceeds the limit of 5.7 MB".into() })
    }

    let image_types = vec!["jpeg", "pjpeg", "png", "webp"];
    let mime_type_err = |type_got: &str| Err(ApiError::Custom { 
        error: "invalid_mime_type".into(), 
        status_code: StatusCode::UNSUPPORTED_MEDIA_TYPE, 
        message: format!(
            "Invalide MIME type: expected file of one of the folowing mime types: {}, but got `{}`",
            image_types.iter().map(|t| format!("`image/{t}`")).collect::<Vec<String>>().join(", "),
            type_got
        )
    });

    if let Some(content_type) = file.content_type.clone() {
        if content_type.type_() != "image" 
            || !image_types.contains(&content_type.subtype().as_str()) 
        {
            return mime_type_err(&format!("{}/{}", content_type.type_(), content_type.subtype()));
        }
    } else {
        return mime_type_err("");
    }

    minio.put_image(&advertiser_id, &campaign_id, file).await?;

    Ok(HttpResponse::NoContent().into())
}

#[utoipa::path(
    tag = "Campaign images",
    operation_id = "get_campaign_image",
    summary = "Получение изображения рекламной кампании",
    description = "Получает изображение к рекламной кампании",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, которому принадлежит кампания."),
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, которой необходимо установить данное изображение.")
    ),
    responses(
        (status = 204, description = "Изображение успешно получено."),
        (status = 404, description = "Рекламодателя, рекламной кампании или с указанным UUID или её изображения не существует.", body = ApiErrorStruct)
    ),
)]
#[get("")]
pub async fn get_handler(
    db: Data<SurrealDB>,
    minio: Data<Minio>,
    Path(path): Path<AdvertiserIdCampaignIdPath>,
) -> Result<HttpResponse, ApiError> {
    let AdvertiserIdCampaignIdPath { advertiser_id, campaign_id } = path;
    Campaign::get_by_id(advertiser_id, campaign_id, &db).await?;

    let (content_type, file) = if let Some((content_type, file)) = minio.get_image(&advertiser_id, &campaign_id).await? {
        (content_type, file)
    }  else {
        return Err(ApiError::NotFound(format!("Image for campaign with UUID `{campaign_id}`")))
    };

    Ok(HttpResponse::Ok()
        .insert_header(("content-type", content_type))
        .body(file))
}

#[utoipa::path(
    tag = "Campaign images",
    operation_id = "delete_campaign_image",
    summary = "Удаление изображения рекламной кампании",
    description = "Удаляет изображение рекламной кампании",
    params(
        ("advertiser_id" = Uuid, description = "UUID рекламодателя, которому принадлежит кампания."),
        ("campaign_id" = Uuid, description = "UUID рекламной кампании, которой необходимо установить данное изображение.")
    ),
    responses(
        (status = 204, description = "Изображение успешно удалено."),
        (status = 404, description = "Рекламодателя или рекламной кампании с указанным UUID не существует.", body = ApiErrorStruct)
    ),
)]
#[delete("")]
pub async fn delete_handler(
    db: Data<SurrealDB>,
    minio: Data<Minio>,
    Path(path): Path<AdvertiserIdCampaignIdPath>,
) -> Result<HttpResponse, ApiError> {
    let AdvertiserIdCampaignIdPath { advertiser_id, campaign_id } = path;
    Campaign::get_by_id(advertiser_id, campaign_id, &db).await?;

    minio.remove_image(&advertiser_id, &campaign_id).await?;

    Ok(HttpResponse::NoContent().into())
}
