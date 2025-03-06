use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Result};
use surrealdb_helper::SurrealDB;

use crate::models::dto::Time;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBTime {
    pub id: RecordId,
    pub current_date: i32,
}

impl DBTime {
    #[allow(non_snake_case)]
    fn DATE_ID() -> (&'static str, &'static str) {
        ("system", "time")
    }

    pub async fn get(db: &SurrealDB) -> Result<Self> {
        Ok(db.0.select(Self::DATE_ID()).await?.unwrap_or_default())
    }

    pub async fn upsert(new_date: Time, db: &SurrealDB) -> Result<Self> {
        Ok(db
            .0
            .upsert(Self::DATE_ID())
            .content(Self::from(new_date))
            .await?
            .unwrap_or_default())
    }
}

impl From<Time> for DBTime {
    fn from(dto: Time) -> Self {
        Self {
            id: RecordId::from(Self::DATE_ID()),
            current_date: dto.current_date,
        }
    }
}

impl Default for DBTime {
    fn default() -> Self {
        Self {
            id: Self::DATE_ID().into(),
            current_date: 1,
        }
    }
}
