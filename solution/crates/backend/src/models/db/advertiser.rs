use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Result};
use surrealdb_helper::SurrealDB;
use uuid::Uuid;

use crate::models::dto::Advertiser;

use super::DBModel;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBAdvertiser {
    pub id: RecordId,
    pub name: String,
}

impl DBModel for DBAdvertiser {
    const TABLE: &str = "advertiser";
}

impl DBAdvertiser {
    pub async fn get(id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        Ok(db.0.select(Self::record_id_from_uuid(&id)).await?)
    }

    pub async fn bulk(advertisers: Vec<Advertiser>, db: &SurrealDB) -> Result<Vec<Self>> {
        Ok(db
            .0
            .query("BEGIN")
            .query(
                r#"
                    FOR $advertiser IN $advertisers {
                        UPSERT ONLY $advertiser.id CONTENT $advertiser;
                    }
                "#,
            )
            .query(
                r#"
                    SELECT * FROM type::table($advertiser_table)
                        WHERE id ∈ $advertisers.map(|$advertiser| $advertiser.id)
                "#,
            )
            .bind((
                "advertisers",
                advertisers.iter().map(Self::from).collect::<Vec<Self>>(),
            ))
            .bind(("advertiser_table", Self::TABLE))
            .query("COMMIT")
            .await?
            .take(1)?)
    }
}

impl From<&Advertiser> for DBAdvertiser {
    fn from(dto: &Advertiser) -> Self {
        Self {
            id: Self::record_id_from_uuid(&dto.id),
            name: dto.name.clone(),
        }
    }
}
