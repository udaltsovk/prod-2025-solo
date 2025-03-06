use actix_web::{dev::ServiceResponse, test::read_body_json};
use builders::{
    AdvertiserBuilder, Buildable, CampaignBuilder, CampaignUpdateBuilder, ClickInfoBuilder,
    ClientBuilder, CreateCampaignBuilder, MLScoreBuilder, StatsBuilder, TargetingBuilder,
};
use derivative::Derivative;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::from_value;
use std::{fmt::Debug, future::Future};
use uuid::Uuid;

pub mod builders;

async fn try_model_from_resp<T: Model>(resp: ServiceResponse) -> Result<T, serde_json::Error> {
    from_value(read_body_json(resp).await)
}

async fn model_from_resp<T: Model>(resp: ServiceResponse) -> T {
    try_model_from_resp(resp).await.unwrap()
}

pub trait Model: DeserializeOwned + PartialEq + Debug {
    fn from_resp(resp: ServiceResponse) -> impl Future<Output = Self> {
        model_from_resp(resp)
    }

    fn try_from_resp(
        resp: ServiceResponse,
    ) -> impl Future<Output = Result<Self, serde_json::Error>> {
        try_model_from_resp(resp)
    }
}

impl<T> Model for Vec<T> where T: Model {}

pub trait OptionalModel {
    fn empty() -> Self;
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Client {
    #[serde(rename = "client_id")]
    pub id: Uuid,
    pub login: String,
    pub age: i64,
    pub location: String,
    pub gender: String,
}
impl Model for Client {}
impl Buildable<ClientBuilder, Self> for Client {}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Advertiser {
    #[serde(rename = "advertiser_id")]
    pub id: Uuid,
    pub name: String,
}
impl Model for Advertiser {}
impl Buildable<AdvertiserBuilder, Self> for Advertiser {}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct MLScore {
    pub client_id: Uuid,
    pub advertiser_id: Uuid,
    pub score: i64,
}
impl Model for MLScore {}
impl Buildable<MLScoreBuilder, Self> for MLScore {}

#[derive(Deserialize, Serialize, Derivative, Clone, Debug)]
#[derivative(PartialEq)]
pub struct Campaign {
    #[serde(rename = "campaign_id")]
    #[derivative(PartialEq = "ignore")]
    pub id: Uuid,
    pub advertiser_id: Uuid,

    #[serde(flatten)]
    pub inner: CreateCampaign,
}
impl Model for Campaign {}
impl Buildable<CampaignBuilder, Self> for Campaign {}
impl Campaign {
    pub fn update(mut self, update: CampaignUpdate) -> Self {
        if let Some(impressions_limit) = update.impressions_limit {
            self.inner.impressions_limit = impressions_limit;
        }
        if let Some(clicks_limit) = update.clicks_limit {
            self.inner.clicks_limit = clicks_limit;
        }
        if let Some(cost_per_impression) = update.cost_per_impression {
            self.inner.cost_per_impression = cost_per_impression;
        }
        if let Some(cost_per_click) = update.cost_per_click {
            self.inner.cost_per_click = cost_per_click;
        }
        if let Some(ad_title) = update.ad_title {
            self.inner.ad_title = ad_title;
        }
        if let Some(ad_text) = update.ad_text {
            self.inner.ad_text = ad_text;
        }
        if let Some(start_date) = update.start_date {
            self.inner.start_date = start_date;
        }
        if let Some(end_date) = update.end_date {
            self.inner.end_date = end_date;
        }
        if let Some(targeting) = update.targeting {
            self.inner.targeting = targeting;
        }
        self
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct CreateCampaign {
    pub impressions_limit: i64,
    pub clicks_limit: i64,
    pub cost_per_impression: f64,
    pub cost_per_click: f64,
    pub ad_title: String,
    pub ad_text: String,
    pub start_date: i64,
    pub end_date: i64,
    pub targeting: Targeting,
}
impl Model for CreateCampaign {}
impl Buildable<CreateCampaignBuilder, Self> for CreateCampaign {}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct CampaignUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions_limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks_limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_impression: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_click: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting: Option<Targeting>,
}
impl Model for CampaignUpdate {}
impl Buildable<CampaignUpdateBuilder, Self> for CampaignUpdate {}
impl OptionalModel for CampaignUpdate {
    fn empty() -> Self {
        Self {
            impressions_limit: None,
            clicks_limit: None,
            cost_per_impression: None,
            cost_per_click: None,
            ad_title: None,
            ad_text: None,
            start_date: None,
            end_date: None,
            targeting: None,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Targeting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_from: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_to: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Model for Targeting {}
impl Buildable<TargetingBuilder, Self> for Targeting {}
impl OptionalModel for Targeting {
    fn empty() -> Self {
        Self {
            gender: None,
            age_from: None,
            age_to: None,
            location: None,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Ad {
    #[serde(rename = "ad_id")]
    id: Uuid,

    #[serde(rename = "ad_title")]
    title: String,

    #[serde(rename = "ad_text")]
    text: String,

    advertiser_id: Uuid,
}
impl From<Campaign> for Ad {
    fn from(campaign: Campaign) -> Self {
        Self {
            id: campaign.id,
            title: campaign.inner.ad_title,
            text: campaign.inner.ad_text,
            advertiser_id: campaign.advertiser_id,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct ClickInfo {
    client_id: Uuid,
}
impl Model for ClickInfo {}
impl Buildable<ClickInfoBuilder, Self> for ClickInfo {}

#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct Stats {
    pub impressions_count: i64,
    pub clicks_count: i64,
    pub conversion: f32,
    pub spent_impressions: f64,
    pub spent_clicks: f64,
    pub spent_total: f64,
}
impl Model for Stats {}
impl Buildable<StatsBuilder, Self> for Stats {}
impl OptionalModel for Stats {
    fn empty() -> Self {
        Stats {
            impressions_count: 0,
            clicks_count: 0,
            conversion: 0.0,
            spent_impressions: 0.0,
            spent_clicks: 0.0,
            spent_total: 0.0,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Clone, Copy, Debug)]
pub struct Time {
    pub current_date: i64,
}
impl Model for Time {}
impl Time {
    pub fn start() -> Self {
        Self { current_date: 1 }
    }

    pub fn increase_day(&mut self) {
        self.current_date += 1
    }

    pub fn decrease_day(&mut self) {
        self.current_date -= 1
    }
}
