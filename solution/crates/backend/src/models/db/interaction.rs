use serde::{Deserialize, Serialize};
use surrealdb::{opt::PatchOp, RecordId, Result};
use surrealdb_helper::SurrealDB;
use uuid::Uuid;

use super::{DBCampaign, DBClient, DBModel, DBRelation};

#[derive(Deserialize, Serialize, Debug)]
pub struct DBInteraction {
    pub id: RecordId,

    #[serde(rename = "in")]
    pub client_id: RecordId,

    #[serde(rename = "out")]
    pub campaign_id: RecordId,

    pub impressed: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicked: Option<i32>,
}
impl DBModel for DBInteraction {
    const TABLE: &str = "interacted_with";
}
impl DBRelation for DBInteraction {}

impl DBInteraction {
    pub async fn create(
        client_id: Uuid,
        campaign_id: RecordId,
        db: &SurrealDB,
    ) -> Result<Option<Self>> {
        let interaction = Self {
            id: Self::relation_id(client_id, DBCampaign::record_id_to_uuid(&campaign_id)),
            client_id: DBClient::record_id_from_uuid(&client_id),
            campaign_id,
            impressed: -1,
            clicked: None,
        };

        Ok(db.0.insert(&interaction.id).relation(interaction).await?)
    }

    pub async fn get(client_id: Uuid, campaign_id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        Ok(db
            .0
            .select(Self::relation_id(client_id, campaign_id))
            .await?)
    }

    pub async fn add_clicked(
        client_id: Uuid,
        campaign_id: Uuid,
        db: &SurrealDB,
    ) -> Result<Option<Self>> {
        Ok(db
            .0
            .update(Self::relation_id(client_id, campaign_id))
            .patch(PatchOp::replace("clicked", -1))
            .await?)
    }
}
