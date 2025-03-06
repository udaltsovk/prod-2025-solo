use serde::{Deserialize, Serialize};
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
    models::{
        db::{DBAdvertiser, DBCampaign, DBModel, DBTargeting},
        url::Pagination,
        TargetingGender,
    },
    routes::ApiError,
    utils::{validation::check_profanity, RoundToDigits},
};

#[derive(PartialEq)]
pub enum CampaignStatus {
    NotStarted,
    Started,
    Ended,
}

use super::{Advertiser, Time};
#[derive(Deserialize, Serialize, ToSchema, Debug)]
#[schema(description = "Объект, представляющий рекламную кампанию.")]
pub struct Campaign {
    #[serde(rename = "campaign_id")]
    /// Уникальный идентификатор рекламной кампании (UUID).
    pub id: Uuid,

    /// UUID рекламодателя, которому принадлежит кампания.
    pub advertiser_id: Uuid,

    #[serde(flatten)]
    pub inner: CreateCampaign,
}

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug)]
#[schema(description = "Объект для создания новой рекламной кампании.")]
#[validate(schema(function = "Self::validate_custom"))]
pub struct CreateCampaign {
    #[schema(examples(1), minimum = 1)]
    #[validate(range(min = 1))]
    /// Задаёт лимит показов для рекламного объявления.
    pub impressions_limit: i32,

    #[schema(examples(0), minimum = 0)]
    #[validate(range(min = 0))]
    /// Задаёт лимит переходов для рекламного объявления.
    pub clicks_limit: i32,

    #[schema(examples(1), exclusive_minimum = 0)]
    #[validate(range(exclusive_min = 0.0))]
    /// Стоимость одного показа объявления.
    pub cost_per_impression: f64,

    #[schema(examples(1), exclusive_minimum = 0)]
    #[validate(range(exclusive_min = 0.0))]

    /// Стоимость одного перехода (клика) по объявлению.
    pub cost_per_click: f64,

    #[validate(custom(function = "check_profanity"))]
    /// Название рекламного объявления.
    pub ad_title: String,

    #[validate(custom(function = "check_profanity"))]
    /// Текст рекламного объявления.
    pub ad_text: String,

    #[schema(examples(1), minimum = 1)]
    #[validate(range(min = 1))]
    /// День начала показа рекламного объявления (включительно).
    pub start_date: i32,

    #[schema(examples(1), minimum = 1)]
    #[validate(range(min = 1))]
    /// День окончания показа рекламного объявления (включительно).
    pub end_date: i32,

    #[schema(examples(Targeting::example))]
    #[validate(nested)]
    pub targeting: Targeting,
}
impl CreateCampaign {
    fn validate_custom(&self) -> Result<(), ValidationError> {
        if self.clicks_limit > self.impressions_limit {
            return Err(ValidationError::new(
                "`clicks_limit` can't be greater than `impressions_limit`",
            ));
        }

        let current_date = Time::get().current_date;
        if self.start_date < current_date {
            return Err(ValidationError::new("`start_date` can't be in the past"));
        }
        if self.end_date < current_date {
            return Err(ValidationError::new("`end_date` can't be in the past"));
        }

        if self.start_date > self.end_date {
            return Err(ValidationError::new(
                "`start_date` can't be greater than `end_date`",
            ));
        }

        Ok(())
    }
}

impl Campaign {
    pub async fn create(
        advertiser_id: Uuid,
        campaign: CreateCampaign,
        db: &SurrealDB,
    ) -> Result<Self, ApiError> {
        Advertiser::get_by_id(advertiser_id, db).await?;

        let campaign = Self {
            id: Uuid::now_v7(),
            advertiser_id,
            inner: campaign,
        };
        Ok((&DBCampaign::create(campaign, db).await?).into())
    }

    pub async fn list(
        advertiser_id: Uuid,
        pagination: Pagination,
        db: &SurrealDB,
    ) -> Result<Vec<Self>, ApiError> {
        Advertiser::get_by_id(advertiser_id, db).await?;

        let size: i64 = match pagination.size {
            Some(size) if size < 57 => size,
            Some(..) => 57,
            None => 7,
        };

        if size == 0 {
            return Ok(vec![]);
        }

        let offset: i64 = pagination.page.unwrap_or(0) * size;
        Ok(DBCampaign::list(advertiser_id, size, offset, db)
            .await?
            .iter()
            .map(Self::from)
            .collect())
    }

    pub async fn get_by_id(
        advertiser_id: Uuid,
        campaign_id: Uuid,
        db: &SurrealDB,
    ) -> Result<Self, ApiError> {
        Advertiser::get_by_id(advertiser_id, db).await?;
        match DBCampaign::get(advertiser_id, campaign_id, &db).await? {
            None => Err(ApiError::NotFound(format!(
                "Campaign with UUID `{}`",
                campaign_id,
            ))),
            Some(campaign) => Ok((&campaign).into()),
        }
    }

    pub async fn get_by_id_unchecked(campaign_id: Uuid, db: &SurrealDB) -> Result<Self, ApiError> {
        match DBCampaign::get_unchecked(campaign_id, &db).await? {
            None => Err(ApiError::NotFound(format!(
                "Campaign with UUID `{}`",
                campaign_id,
            ))),
            Some(campaign) => Ok((&campaign).into()),
        }
    }

    pub async fn update(
        advertiser_id: Uuid,
        campaign_id: Uuid,
        campaign_update: CampaignUpdate,
        db: &SurrealDB,
    ) -> Result<Self, ApiError> {
        let campaign_status = Self::get_by_id(advertiser_id, campaign_id, db)
            .await?
            .get_status();
        let unable_to_change_field_err =
            |field_name: &str| Err(ApiError::CampaignStarted(field_name.to_string()));

        if campaign_update.impressions_limit.is_some()
            && campaign_status != CampaignStatus::NotStarted
        {
            return unable_to_change_field_err("impressions_limit");
        }
        if campaign_update.clicks_limit.is_some() && campaign_status != CampaignStatus::NotStarted {
            return unable_to_change_field_err("clicks_limit");
        }
        if campaign_update.start_date.is_some() && campaign_status != CampaignStatus::NotStarted {
            return unable_to_change_field_err("start_date");
        }
        if campaign_update.end_date.is_some() && campaign_status != CampaignStatus::NotStarted {
            return unable_to_change_field_err("end_date");
        }
        if campaign_update.targeting.is_some() && campaign_status != CampaignStatus::NotStarted {
            return unable_to_change_field_err("targeting");
        }

        let campaign = DBCampaign::update(advertiser_id, campaign_id, campaign_update, db)
            .await?
            .unwrap();

        Ok((&campaign).into())
    }

    pub async fn delete(
        advertiser_id: Uuid,
        campaign_id: Uuid,
        db: &SurrealDB,
    ) -> Result<Self, ApiError> {
        Advertiser::get_by_id(advertiser_id, db).await?;
        match DBCampaign::delete(advertiser_id, campaign_id, db).await? {
            None => Err(ApiError::NotFound(format!(
                "Campaign with UUID `{}`",
                campaign_id,
            ))),
            Some(campaign) => Ok((&campaign).into()),
        }
    }

    pub fn get_status(&self) -> CampaignStatus {
        match (
            self.inner.start_date,
            Time::get().current_date,
            self.inner.end_date,
        ) {
            (sd, cd, ..) if cd < sd => CampaignStatus::NotStarted,
            (.., cd, ed) if cd > ed => CampaignStatus::Ended,
            _ => CampaignStatus::Started,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug)]
#[schema(
    description = "Объект для обновления параметров кампании, которые разрешено изменять до старта кампании."
)]
#[validate(schema(function = "Self::validate_custom"))]
pub struct CampaignUpdate {
    #[schema(examples(1), minimum = 1)]
    #[validate(range(min = 1))]
    /// Задаёт лимит показов для рекламного объявления.
    pub impressions_limit: Option<i32>,

    #[schema(examples(0), minimum = 0)]
    #[validate(range(min = 0))]
    /// Задаёт лимит переходов для рекламного объявления.
    pub clicks_limit: Option<i32>,

    #[schema(examples(1), exclusive_minimum = 0)]
    #[validate(range(exclusive_min = 0.0))]
    /// Новая стоимость одного показа объявления.
    pub cost_per_impression: Option<f64>,

    #[schema(examples(1), exclusive_minimum = 0)]
    #[validate(range(exclusive_min = 0.0))]
    /// Новая стоимость одного перехода (клика) по объявлению.
    pub cost_per_click: Option<f64>,

    #[validate(custom(function = "check_profanity"))]
    /// Новое название рекламного объявления.
    pub ad_title: Option<String>,

    #[validate(custom(function = "check_profanity"))]
    /// Новый текст рекламного объявления.
    pub ad_text: Option<String>,

    #[schema(examples(1), minimum = 1)]
    #[validate(range(min = 1))]
    /// День начала показа рекламного объявления (включительно).
    pub start_date: Option<i32>,

    #[schema(examples(1), minimum = 1)]
    #[validate(range(min = 1))]
    /// День окончания показа рекламного объявления (включительно).
    pub end_date: Option<i32>,

    #[schema(examples(Targeting::example))]
    #[validate(nested)]
    pub targeting: Option<Targeting>,
}
impl CampaignUpdate {
    pub fn validate_custom(&self) -> Result<(), ValidationError> {
        if self.clicks_limit.unwrap_or(0) > self.impressions_limit.unwrap_or(i32::MAX) {
            return Err(ValidationError::new(
                "`clicks_limit` can't be greater than `impressions_limit`",
            ));
        }

        let current_date = Time::get().current_date;
        if self.start_date.unwrap_or(current_date) < current_date {
            return Err(ValidationError::new("`start_date` can't be in the past"));
        }
        if self.end_date.unwrap_or(current_date) < current_date {
            return Err(ValidationError::new("`end_date` can't be in the past"));
        }

        if self.start_date.unwrap_or(0) > self.end_date.unwrap_or(i32::MAX) {
            return Err(ValidationError::new(
                "`start_date` can't be greater than `end_date`",
            ));
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug)]
#[schema(description = "Объект, описывающий настройки таргетирования для рекламной кампании.")]
#[validate(schema(function = "Self::validate_custom"))]
pub struct Targeting {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable, examples(TargetingGender::MALE))]
    /// Пол аудитории для показа объявления (MALE, FEMALE или ALL).
    pub gender: Option<TargetingGender>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable, examples(0), minimum = 0)]
    #[validate(range(min = 0))]
    /// Минимальный возраст аудитории (включительно) для показа объявления.
    pub age_from: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable, examples(0))]
    #[validate(range(min = 0, max = 200))]
    /// Максимальный возраст аудитории (включительно) для показа объявления.
    pub age_to: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable, examples("string"))]
    #[validate(custom(function = "check_profanity"))]
    /// Локация аудитории, для которой будет показано объявление.
    pub location: Option<String>,
}

impl Targeting {
    pub fn validate_custom(&self) -> Result<(), ValidationError> {
        if self.age_from.is_some() && self.age_to.is_some() && self.age_from > self.age_to {
            return Err(ValidationError::new(
                "`age_from` must be less than `age_to`",
            ));
        }

        Ok(())
    }

    pub fn example() -> Self {
        Self {
            gender: Some(TargetingGender::MALE),
            age_from: Some(0),
            age_to: Some(0),
            location: Some("string".to_string()),
        }
    }

    pub fn is_none(targeting: &Option<Self>) -> bool {
        targeting.as_ref().map_or(true, |t| {
            t.gender.is_none() && t.age_from.is_none() && t.age_to.is_none() && t.location.is_none()
        })
    }
}

impl From<DBTargeting> for Targeting {
    fn from(db: DBTargeting) -> Self {
        Self {
            gender: db.gender,
            age_from: db.age_from,
            age_to: db.age_to,
            location: db.location,
        }
    }
}

impl From<&DBCampaign> for Campaign {
    fn from(db: &DBCampaign) -> Self {
        Self {
            id: DBCampaign::record_id_to_uuid(&db.id),
            advertiser_id: DBAdvertiser::record_id_to_uuid(&db.advertiser_id),
            inner: CreateCampaign {
                impressions_limit: db.impressions_limit,
                clicks_limit: db.clicks_limit,
                cost_per_impression: db.cost_per_impression.round_to_digits(2),
                cost_per_click: db.cost_per_click.round_to_digits(2),
                ad_title: db.ad_title.clone(),
                ad_text: db.ad_text.clone(),
                start_date: db.start_date,
                end_date: db.end_date,
                targeting: Targeting::from(db.targeting.clone()),
            },
        }
    }
}
