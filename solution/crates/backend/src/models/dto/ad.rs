use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    models::db::{DBAdvertiser, DBCampaign, DBInteraction, DBModel},
    routes::ApiError,
};

use super::{Campaign, Client};

#[derive(Serialize, ToSchema, Debug)]
#[schema(
    description = "Объект, представляющий рекламное объявление, которое показывается клиенту."
)]
pub struct Ad {
    #[serde(rename = "ad_id")]
    /// Уникальный идентификатор рекламного объявления (всегда совпадает с id рекламной кампании).
    id: Uuid,

    #[serde(rename = "ad_title")]
    /// Название рекламного объявления.
    title: String,

    #[serde(rename = "ad_text")]
    /// Текст рекламного объявления, который видит клиент.
    text: String,

    /// UUID рекламодателя, которому принадлежит объявление.
    advertiser_id: Uuid,
}

impl Ad {
    pub async fn get_for_client(client_id: Uuid, db: &SurrealDB) -> Result<HttpResponse, ApiError> {
        Client::get_by_id(client_id, db).await?;
        Ok(match DBCampaign::get_ad_for_client(client_id, db).await? {
            None => HttpResponse::NoContent().into(),
            Some(campaign) => {
                DBInteraction::create(client_id, campaign.id.clone(), db).await?;
                HttpResponse::Ok().json(Self::from(campaign))
            }
        })
    }

    pub async fn record_click(
        client_id: Uuid,
        ad_id: Uuid,
        db: &SurrealDB,
    ) -> Result<(), ApiError> {
        Client::get_by_id(client_id, db).await?;
        Campaign::get_by_id_unchecked(ad_id, db).await?;

        let interaction = match DBInteraction::get(client_id, ad_id, db).await? {
            None => {
                return Err(ApiError::Custom {
                    error: "no_view".into(),
                    status_code: StatusCode::CONFLICT,
                    message: "This client haven't viewed this ad yet".into(),
                })
            }
            Some(interaction) => interaction,
        };

        if interaction.clicked.is_none() {
            DBInteraction::add_clicked(client_id, ad_id, db).await?;
        }

        Ok(())
    }
}

impl From<DBCampaign> for Ad {
    fn from(db: DBCampaign) -> Self {
        Self {
            id: DBCampaign::record_id_to_uuid(&db.id),
            title: db.ad_title,
            text: db.ad_text,
            advertiser_id: DBAdvertiser::record_id_to_uuid(&db.advertiser_id),
        }
    }
}
