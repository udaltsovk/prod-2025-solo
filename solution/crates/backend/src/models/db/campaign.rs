use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::{RecordId, Result};
use surrealdb_helper::SurrealDB;
use uuid::Uuid;

use crate::{
    models::{
        dto::{Campaign, CampaignUpdate, Targeting},
        TargetingGender,
    },
    utils::RoundToDigits,
};

use super::{DBAdvertiser, DBClient, DBModel};

#[derive(Deserialize, Serialize, Debug)]
pub struct DBCampaign {
    pub id: RecordId,
    pub advertiser_id: RecordId,
    pub impressions_limit: i32,
    pub clicks_limit: i32,
    pub cost_per_impression: f64,
    pub cost_per_click: f64,
    pub ad_title: String,
    pub ad_text: String,
    pub start_date: i32,
    pub end_date: i32,
    pub targeting: DBTargeting,
    pub is_active: bool,
}

impl DBModel for DBCampaign {
    const TABLE: &str = "campaign";
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBTargeting {
    pub gender: Option<TargetingGender>,
    pub age_from: Option<i32>,
    pub age_to: Option<i32>,
    pub location: Option<String>,
}

impl DBCampaign {
    pub async fn create(campaign: Campaign, db: &SurrealDB) -> Result<Self> {
        let campaign = Self::from(campaign);
        Ok(db.0.create(&campaign.id).content(campaign).await?.unwrap())
    }

    pub async fn list(
        advertiser_id: Uuid,
        limit: i64,
        offset: i64,
        db: &SurrealDB,
    ) -> Result<Vec<Self>> {
        Ok(db
            .0
            .query(
                r#"
                    SELECT * FROM type::table($campaign_table)
                        WHERE advertiser_id = type::thing($advertiser_table, $advertiser_id)
                        LIMIT <number>($limit)
                        START <number>($offset)
                "#,
            )
            .bind(json!({
                "campaign_table": Self::TABLE,
                "advertiser_table": DBAdvertiser::TABLE,
                "advertiser_id": advertiser_id,
                "limit": limit,
                "offset": offset
            }))
            .await?
            .take(0)?)
    }

    pub async fn get(advertiser_id: Uuid, id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        Ok(db
            .0
            .query(
                r#"
                    SELECT * FROM ONLY type::thing($campaign_table, $campaign_id) 
                        WHERE advertiser_id = type::thing($advertiser_table, $advertiser_id)
                "#,
            )
            .bind(json!({
                "campaign_table": Self::TABLE,
                "campaign_id": id,
                "advertiser_table": DBAdvertiser::TABLE,
                "advertiser_id": advertiser_id,
            }))
            .await?
            .take(0)?)
    }

    pub async fn get_unchecked(id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        Ok(db.0.select(Self::record_id_from_uuid(&id)).await?)
    }

    pub async fn update(
        advertiser_id: Uuid,
        id: Uuid,
        campaign_update: CampaignUpdate,
        db: &SurrealDB,
    ) -> Result<Option<Self>> {
        let campaign_update = DBCampaignUpdate::from(campaign_update);
        Ok(db
            .0
            .query(
                r#"
                    UPDATE ONLY type::thing($campaign_table, $campaign_id) 
                        MERGE $campaign_update
                        WHERE advertiser_id = type::thing($advertiser_table, $advertiser_id)
                        RETURN AFTER
                "#,
            )
            .bind(json!({
                "campaign_table": Self::TABLE,
                "campaign_id": id,
                "advertiser_table": DBAdvertiser::TABLE,
                "advertiser_id": advertiser_id,
                "campaign_update": campaign_update
            }))
            .await?
            .take(0)?)
    }

    pub async fn delete(advertiser_id: Uuid, id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        Ok(db
            .0
            .query(
                r#"
                    DELETE type::thing($campaign_table, $campaign_id) 
                        WHERE advertiser_id = type::thing($advertiser_table, $advertiser_id)
                        RETURN BEFORE
                "#,
            )
            .bind(json!({
                "campaign_table": Self::TABLE,
                "campaign_id": id,
                "advertiser_table": DBAdvertiser::TABLE,
                "advertiser_id": advertiser_id,
            }))
            .await?
            .take(0)?)
    }

    pub async fn get_ad_for_client(client_id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        Ok(db
            .0
            .query(
                r#"
                    SELECT *,
                        (fn::score_campaign($client, $this.id)) as score
                    FROM ONLY type::table($campaign_table)
                        WHERE is_active 
                            && (targeting.gender ?? "ALL") ∈ ["ALL", $client.gender]
                            && (targeting.age_from ?? 0) <= $client.age && $client.age <= (targeting.age_to ?? math::inf)
                            && $client.location == (targeting.location ?? $client.location)
                        ORDER BY score
                        LIMIT 1
                "#
            )
            .bind((
                "client", 
                DBClient::record_id_from_uuid(&client_id)
            ))
            .bind((
                "campaign_table", Self::TABLE
            ))
            .await?
            .take(0)?)
    }
}

#[derive(Serialize, Debug)]
pub struct DBCampaignUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions_limit: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks_limit: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_impression: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_click: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting: Option<DBTargeting>,
}

impl From<CampaignUpdate> for DBCampaignUpdate {
    fn from(dto: CampaignUpdate) -> Self {
        Self {
            impressions_limit: dto.impressions_limit,
            clicks_limit: dto.clicks_limit,
            cost_per_impression: dto.cost_per_impression.map(|c| c.round_to_digits(2)),
            cost_per_click: dto.cost_per_click.map(|c| c.round_to_digits(2)),
            ad_title: dto.ad_title,
            ad_text: dto.ad_text,
            start_date: dto.start_date,
            end_date: dto.end_date,
            targeting: dto.targeting.map(DBTargeting::from),
        }
    }
}

impl From<Targeting> for DBTargeting {
    fn from(dto: Targeting) -> Self {
        Self {
            gender: dto.gender,
            age_from: dto.age_from,
            age_to: dto.age_to,
            location: dto.location,
        }
    }
}

impl From<Campaign> for DBCampaign {
    fn from(dto: Campaign) -> Self {
        Self {
            id: Self::record_id_from_uuid(&dto.id),
            advertiser_id: DBAdvertiser::record_id_from_uuid(&dto.advertiser_id),
            impressions_limit: dto.inner.impressions_limit,
            clicks_limit: dto.inner.clicks_limit,
            cost_per_impression: dto.inner.cost_per_impression.round_to_digits(2),
            cost_per_click: dto.inner.cost_per_click.round_to_digits(2),
            ad_title: dto.inner.ad_title,
            ad_text: dto.inner.ad_text,
            start_date: dto.inner.start_date,
            end_date: dto.inner.end_date,
            targeting: DBTargeting::from(dto.inner.targeting),
            is_active: false,
        }
    }
}
