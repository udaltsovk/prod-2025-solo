use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::models::db::DBModel;

use super::{DBStats, DBStatsModel};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBStatsAdvertiser {
    pub id: RecordId,
    pub advertiser_id: RecordId,

    #[serde(flatten)]
    pub stats: DBStats,
}
impl DBModel for DBStatsAdvertiser {
    const TABLE: &str = "stats_advertiser";
}
impl DBStatsModel for DBStatsAdvertiser {
    fn stats(&self) -> DBStats {
        self.stats.clone()
    }
}
