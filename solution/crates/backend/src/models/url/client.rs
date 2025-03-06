use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Deserialize, IntoParams, Debug)]
pub struct ClientIdPath {
    pub client_id: Uuid,
}
