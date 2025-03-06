use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::Result;
use surrealdb_helper::SurrealDB;
use uuid::Uuid;

use super::DBModel;

mod advertiser;
mod campaign;

pub use advertiser::DBStatsAdvertiser;
pub use campaign::DBStatsCampaign;

#[async_trait]
pub trait DBStatsModel: DBModel + DeserializeOwned {
    fn stats(&self) -> DBStats;

    fn total(&self) -> DBStatsBasic {
        self.stats().total
    }

    fn daily(&self) -> Vec<DBStatsBasic> {
        let stats = self.stats();
        let current = stats.current;
        let mut daily = stats.daily;
        daily.push(current);
        daily
    }

    async fn get(id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        db.0.select(Self::record_id_from_uuid(&id)).await
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBStats {
    current: DBStatsBasic,
    total: DBStatsBasic,
    daily: Vec<DBStatsBasic>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBStatsBasic {
    pub impressions_count: u64,
    pub clicks_count: u64,
    pub conversion: f32,
    pub spent_impressions: f64,
    pub spent_clicks: f64,
    pub spent_total: f64,
}
