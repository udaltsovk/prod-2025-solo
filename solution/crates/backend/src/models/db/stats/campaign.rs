use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::models::db::DBModel;

use super::{DBStats, DBStatsModel};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBStatsCampaign {
    pub id: RecordId,
    pub campaign_id: RecordId,

    #[serde(flatten)]
    pub stats: DBStats,
}
impl DBModel for DBStatsCampaign {
    const TABLE: &str = "stats_campaign";
}
impl DBStatsModel for DBStatsCampaign {
    fn stats(&self) -> DBStats {
        self.stats.clone()
    }
}
