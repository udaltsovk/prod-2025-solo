use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod db;
pub mod dto;
mod gender;
pub mod url;

pub use gender::{Gender, TargetingGender};

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ApiError {
    pub error: String,
    pub description: String,
}
