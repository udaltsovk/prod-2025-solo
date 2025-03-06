use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams, Debug)]
pub struct AdvertiserIdCampaignIdPath {
    pub advertiser_id: Uuid,
    pub campaign_id: Uuid,
}

#[derive(Deserialize, IntoParams, Debug)]
pub struct CampaignIdPath {
    pub campaign_id: Uuid,
}
