use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Result};
use surrealdb_helper::SurrealDB;

use crate::models::dto::MLScore;

use super::{DBAdvertiser, DBClient, DBModel, DBRelation};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBMLScore {
    pub id: RecordId,

    #[serde(rename = "in")]
    pub advertiser_id: RecordId,

    #[serde(rename = "out")]
    pub client_id: RecordId,

    pub score: i32,
}
impl DBModel for DBMLScore {
    const TABLE: &str = "scored";
}
impl DBRelation for DBMLScore {}

impl DBMLScore {
    pub async fn upsert(score: MLScore, db: &SurrealDB) -> Result<Self> {
        let score = Self::from(score);
        let result: Option<Self> = db.0
            .query(
                r#"
                    RELATE ONLY (type::record($advertiser_id))->(type::record($score_id))->(type::record($client_id))
                        SET score = <number>$score
                "#
            )
            .bind(("advertiser_id", score.advertiser_id))
            .bind(("client_id", score.client_id))
            .bind(("score_id", score.id))
            .bind(("score", score.score))
            .await?
            .take(0)?;
        Ok(result.unwrap())
    }
}

impl From<MLScore> for DBMLScore {
    fn from(dto: MLScore) -> Self {
        Self {
            id: Self::relation_id(dto.advertiser_id, dto.client_id),
            client_id: DBClient::record_id_from_uuid(&dto.client_id),
            advertiser_id: DBAdvertiser::record_id_from_uuid(&dto.advertiser_id),
            score: dto.score,
        }
    }
}
