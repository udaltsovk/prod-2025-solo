use serde::Serialize;
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    models::{
        db::stats::{DBStatsAdvertiser, DBStatsBasic, DBStatsCampaign, DBStatsModel},
        dto::{Advertiser, Campaign},
    },
    routes::ApiError,
};

use super::campaign::CampaignStatus;

#[derive(Serialize, ToSchema, Debug)]
#[schema(
    description = "Объект, содержащий агрегированную статистику для рекламной кампании или рекламодателя."
)]
pub struct Stats {
    #[schema(examples(0), minimum = 0)]
    /// Общее количество уникальных показов рекламного объявления.
    pub impressions_count: u64,

    #[schema(examples(0), minimum = 0)]
    /// Общее количество уникальных переходов (кликов) по рекламному объявлению.
    pub clicks_count: u64,

    #[schema(examples(0), minimum = 0, maximum = 100)]
    /// Коэффициент конверсии, вычисляемый как (clicks_count / impressions_count * 100) в процентах.
    pub conversion: f32,

    #[schema(examples(0), minimum = 0)]
    /// Сумма денег, потраченная на показы рекламного объявления.
    pub spent_impressions: f64,

    #[schema(examples(0), minimum = 0)]
    /// Сумма денег, потраченная на переходы (клики) по рекламному объявлению.
    pub spent_clicks: f64,

    #[schema(examples(0), minimum = 0)]
    /// Общая сумма денег, потраченная на кампанию (показы и клики).
    pub spent_total: f64,
}

impl Stats {
    pub async fn campaign(campaign_id: Uuid, db: &SurrealDB) -> Result<Self, ApiError> {
        Campaign::get_by_id_unchecked(campaign_id, db).await?;
        match DBStatsCampaign::get(campaign_id, db).await? {
            None => Err(ApiError::NotFound(format!(
                "Stats for campaign with UUID `{}`",
                campaign_id,
            ))),
            Some(stats) => Ok(Self::from_db_total(stats)),
        }
    }

    pub async fn campaign_daily(campaign_id: Uuid, db: &SurrealDB) -> Result<Vec<Self>, ApiError> {
        let campaign = Campaign::get_by_id_unchecked(campaign_id, db).await?;
        if campaign.get_status() == CampaignStatus::NotStarted {
            return Ok(vec![]);
        }
        match DBStatsCampaign::get(campaign_id, db).await? {
            None => Err(ApiError::NotFound(format!(
                "Stats for campaign with UUID `{}`",
                campaign_id,
            ))),
            Some(stats) => Ok(Self::from_db_daily(stats)),
        }
    }

    pub async fn advertiser(advertiser_id: Uuid, db: &SurrealDB) -> Result<Self, ApiError> {
        Advertiser::get_by_id(advertiser_id, db).await?;
        match DBStatsAdvertiser::get(advertiser_id, db).await? {
            None => Err(ApiError::NotFound(format!(
                "Stats for campaign with UUID `{}`",
                advertiser_id,
            ))),
            Some(stats) => Ok(Self::from_db_total(stats)),
        }
    }

    pub async fn advertiser_daily(
        advertiser_id: Uuid,
        db: &SurrealDB,
    ) -> Result<Vec<Self>, ApiError> {
        Advertiser::get_by_id(advertiser_id, db).await?;
        match DBStatsAdvertiser::get(advertiser_id, db).await? {
            None => Err(ApiError::NotFound(format!(
                "Stats for campaign with UUID `{}`",
                advertiser_id,
            ))),
            Some(stats) => Ok(Self::from_db_daily(stats)),
        }
    }

    pub fn from_db_total(db: impl DBStatsModel) -> Self {
        Self::from(&db.total())
    }

    pub fn from_db_daily(db: impl DBStatsModel) -> Vec<Self> {
        db.daily().iter().map(Self::from).collect()
    }
}

impl From<&DBStatsBasic> for Stats {
    fn from(db: &DBStatsBasic) -> Self {
        Self {
            impressions_count: db.impressions_count,
            clicks_count: db.clicks_count,
            conversion: db.conversion,
            spent_impressions: db.spent_impressions,
            spent_clicks: db.spent_clicks,
            spent_total: db.spent_total,
        }
    }
}
