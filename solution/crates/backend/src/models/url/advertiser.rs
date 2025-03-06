use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams, Debug)]
pub struct AdvertiserIdPath {
    pub advertiser_id: Uuid,
}
