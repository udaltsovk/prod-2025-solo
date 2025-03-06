mod ad;
mod advertiser;
mod campaign;
mod client;
mod pagination;

pub use ad::AdIdPath;
pub use advertiser::AdvertiserIdPath;
pub use campaign::{AdvertiserIdCampaignIdPath, CampaignIdPath};
pub use client::ClientIdPath;
pub use pagination::Pagination;
