use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams, Debug)]
pub struct AdIdPath {
    pub ad_id: Uuid,
}
