use serde::{Deserialize, Serialize};
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::db::{DBAdvertiser, DBModel},
    routes::ApiError,
    utils::validation::check_profanity,
};

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug)]
#[schema(description = "Объект, представляющий рекламодателя.")]
pub struct Advertiser {
    #[serde(rename = "advertiser_id")]
    /// Уникальный идентификатор рекламодателя (UUID).
    pub id: Uuid,

    #[validate(custom(function = "check_profanity"))]
    /// Название рекламодателя.
    pub name: String,
}

impl Advertiser {
    pub async fn get_by_id(advertiser_id: Uuid, db: &SurrealDB) -> Result<Self, ApiError> {
        match DBAdvertiser::get(advertiser_id, db).await? {
            None => Err(ApiError::NotFound(format!(
                "Advertiser with UUID `{}`",
                advertiser_id
            ))),
            Some(client) => Ok((&client).into()),
        }
    }

    pub async fn upsert(body: Vec<Self>, db: &SurrealDB) -> Result<Vec<Self>, ApiError> {
        Ok(DBAdvertiser::bulk(body, db)
            .await?
            .iter()
            .map(Self::from)
            .collect())
    }
}

impl From<&DBAdvertiser> for Advertiser {
    fn from(db: &DBAdvertiser) -> Self {
        Self {
            id: DBAdvertiser::record_id_to_uuid(&db.id),
            name: db.name.clone(),
        }
    }
}
