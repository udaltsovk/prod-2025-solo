use serde::{Deserialize, Serialize};
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::db::{DBAdvertiser, DBClient, DBMLScore, DBModel},
    routes::ApiError,
};

use super::{Advertiser, Client};

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug)]
#[schema(description = "Объект, представляющий ML скор для пары клиент-рекламодатель.")]
pub struct MLScore {
    /// UUID клиента, для которого рассчитывается ML скор.
    pub client_id: Uuid,

    /// UUID рекламодателя, для которого рассчитывается ML скор.
    pub advertiser_id: Uuid,

    #[schema(example = 0, minimum = 0)]
    #[validate(range(min = 0))]
    /// Целочисленное значение ML скора; чем больше – тем выше релевантность.
    pub score: i32,
}

impl MLScore {
    pub async fn upsert(self, db: &SurrealDB) -> Result<Self, ApiError> {
        Advertiser::get_by_id(self.advertiser_id, db).await?;
        Client::get_by_id(self.client_id, db).await?;

        Ok(DBMLScore::upsert(self, db).await?.into())
    }
}

impl From<DBMLScore> for MLScore {
    fn from(db: DBMLScore) -> Self {
        Self {
            client_id: DBClient::record_id_to_uuid(&db.client_id),
            advertiser_id: DBAdvertiser::record_id_to_uuid(&db.advertiser_id),
            score: db.score,
        }
    }
}
