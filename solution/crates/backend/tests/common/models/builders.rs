use backend::{
    models::{Gender, TargetingGender},
    utils::RoundToDigits,
};
use rand::{distr::uniform::SampleUniform, random_bool, rng, seq::IndexedRandom, Rng};
use std::sync::atomic::{AtomicU32, Ordering};
use uuid::Uuid;

use super::{
    Advertiser, Campaign, CampaignUpdate, ClickInfo, Client, CreateCampaign, MLScore, Stats,
    Targeting,
};

static CLIENT_COUNTER: AtomicU32 = AtomicU32::new(0);
static ADVERTISER_COUNTER: AtomicU32 = AtomicU32::new(0);
static CAMPAIGN_COUNTER: AtomicU32 = AtomicU32::new(0);

pub trait Builder<T> {
    fn new() -> Self;

    fn build(self) -> T;
}

pub trait Buildable<B: Builder<T>, T> {
    fn builder() -> B {
        B::new()
    }

    fn new() -> T {
        Self::builder().build()
    }
}

pub struct ClientBuilder {
    id: Option<Uuid>,
    login: Option<String>,
    age: Option<i64>,
    location: Option<String>,
    gender: Option<String>,
}
impl Builder<Client> for ClientBuilder {
    fn new() -> Self {
        Self {
            id: None,
            login: None,
            age: None,
            location: None,
            gender: None,
        }
    }

    fn build(self) -> Client {
        let counter = CLIENT_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut rng = rng();

        Client {
            id: self.id.unwrap_or_else(Uuid::now_v7),
            login: self.login.unwrap_or_else(|| format!("user_{}", counter)),
            age: self.age.unwrap_or_else(|| rng.random_range(13..=80)),
            location: self
                .location
                .unwrap_or_else(|| generate_location(LocationType::City)),
            gender: self.gender.unwrap_or_else(|| {
                if counter % 2 == 0 {
                    Gender::MALE
                } else {
                    Gender::FEMALE
                }
                .to_string()
            }),
        }
    }
}
impl ClientBuilder {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_login(mut self, login: impl Into<String>) -> Self {
        self.login = Some(login.into());
        self
    }

    pub fn with_age(mut self, age: i64) -> Self {
        self.age = Some(age);
        self
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn with_gender(mut self, gender: &str) -> Self {
        self.gender = Some(gender.to_string());
        self
    }
}

pub struct AdvertiserBuilder {
    id: Option<Uuid>,
    name: Option<String>,
}
impl Builder<Advertiser> for AdvertiserBuilder {
    fn new() -> Self {
        Self {
            id: None,
            name: None,
        }
    }

    fn build(self) -> Advertiser {
        let counter = ADVERTISER_COUNTER.fetch_add(1, Ordering::SeqCst);

        Advertiser {
            id: self.id.unwrap_or_else(Uuid::now_v7),
            name: self
                .name
                .unwrap_or_else(|| format!("Advertiser_{}", counter)),
        }
    }
}
impl AdvertiserBuilder {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

pub struct MLScoreBuilder {
    client_id: Option<Uuid>,
    advertiser_id: Option<Uuid>,
    score: Option<i64>,
}
impl Builder<MLScore> for MLScoreBuilder {
    fn new() -> Self {
        Self {
            client_id: None,
            advertiser_id: None,
            score: None,
        }
    }

    fn build(self) -> MLScore {
        let mut rng = rng();

        MLScore {
            client_id: self.client_id.unwrap_or_else(Uuid::now_v7),
            advertiser_id: self.advertiser_id.unwrap_or_else(Uuid::now_v7),
            score: self
                .score
                .unwrap_or_else(|| rng.random_range(0..=i32::MAX) as i64),
        }
    }
}
impl MLScoreBuilder {
    pub fn with_client_id(mut self, client_id: Uuid) -> Self {
        self.client_id = Some(client_id);
        self
    }

    pub fn with_advertiser_id(mut self, advertiser_id: Uuid) -> Self {
        self.advertiser_id = Some(advertiser_id);
        self
    }

    pub fn with_score(mut self, score: i64) -> Self {
        self.score = Some(score);
        self
    }
}

pub struct CampaignBuilder {
    id: Option<Uuid>,
    advertiser_id: Option<Uuid>,
    inner: Option<CreateCampaign>,
}
impl Builder<Campaign> for CampaignBuilder {
    fn new() -> Self {
        Self {
            id: None,
            advertiser_id: None,
            inner: None,
        }
    }

    fn build(self) -> Campaign {
        Campaign {
            id: self.id.unwrap_or_else(Uuid::now_v7),
            advertiser_id: self.advertiser_id.unwrap_or_else(Uuid::now_v7),
            inner: self.inner.unwrap_or_else(CreateCampaign::new),
        }
    }
}
impl CampaignBuilder {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_advertiser_id(mut self, advertiser_id: Uuid) -> Self {
        self.advertiser_id = Some(advertiser_id);
        self
    }

    pub fn with_inner(mut self, inner: CreateCampaign) -> Self {
        self.inner = Some(inner);
        self
    }
}

pub struct CreateCampaignBuilder {
    impressions_limit: Option<i64>,
    clicks_limit: Option<i64>,
    cost_per_impression: Option<f64>,
    cost_per_click: Option<f64>,
    ad_title: Option<String>,
    ad_text: Option<String>,
    start_date: Option<i64>,
    end_date: Option<i64>,
    targeting: Option<Targeting>,
}
impl Builder<CreateCampaign> for CreateCampaignBuilder {
    fn new() -> Self {
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

    fn build(self) -> CreateCampaign {
        let counter = CAMPAIGN_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut rng = rng();

        let (clicks_limit, impressions_limit) = generate_validated(
            self.clicks_limit,
            10,
            10_000,
            self.impressions_limit,
            100,
            100_000,
        );
        let (start_date, end_date) =
            generate_validated(self.start_date, 1, 100, self.end_date, 1, 100);

        CreateCampaign {
            impressions_limit,
            clicks_limit,
            cost_per_impression: self
                .cost_per_impression
                .unwrap_or_else(|| rng.random_range(0.01..10.0))
                .round_to_digits(2),
            cost_per_click: self
                .cost_per_click
                .unwrap_or_else(|| rng.random_range(1.0..100.0))
                .round_to_digits(2),
            ad_title: self
                .ad_title
                .unwrap_or_else(|| format!("Campaign_{}", counter)),
            ad_text: self
                .ad_text
                .unwrap_or_else(|| format!("Ad content for Campaign_{}", counter)),
            start_date,
            end_date,
            targeting: self.targeting.unwrap_or_else(Targeting::new),
        }
    }
}
impl CreateCampaignBuilder {
    pub fn with_impressions_limit(mut self, impressions_limit: i64) -> Self {
        self.impressions_limit = Some(impressions_limit);
        self
    }

    pub fn with_clicks_limit(mut self, clicks_limit: i64) -> Self {
        self.clicks_limit = Some(clicks_limit);
        self
    }

    pub fn with_cost_per_impression(mut self, cost_per_impression: f64) -> Self {
        self.cost_per_impression = Some(cost_per_impression);
        self
    }

    pub fn with_cost_per_click(mut self, cost_per_click: f64) -> Self {
        self.cost_per_click = Some(cost_per_click);
        self
    }

    pub fn with_ad_title(mut self, ad_title: impl Into<String>) -> Self {
        self.ad_title = Some(ad_title.into());
        self
    }

    pub fn with_ad_text(mut self, ad_text: impl Into<String>) -> Self {
        self.ad_text = Some(ad_text.into());
        self
    }

    pub fn with_start_date(mut self, start_date: i64) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: i64) -> Self {
        self.end_date = Some(end_date);
        self
    }

    pub fn with_targeting(mut self, targeting: Targeting) -> Self {
        self.targeting = Some(targeting);
        self
    }
}

pub struct CampaignUpdateBuilder {
    impressions_limit: Option<Option<i64>>,
    clicks_limit: Option<Option<i64>>,
    cost_per_impression: Option<Option<f64>>,
    cost_per_click: Option<Option<f64>>,
    ad_title: Option<Option<String>>,
    ad_text: Option<Option<String>>,
    start_date: Option<Option<i64>>,
    end_date: Option<Option<i64>>,
    targeting: Option<Option<Targeting>>,
}
impl Builder<CampaignUpdate> for CampaignUpdateBuilder {
    fn new() -> Self {
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

    fn build(self) -> CampaignUpdate {
        let counter = CAMPAIGN_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut rng = rng();

        let (clicks_limit, impressions_limit) = generate_validated_option(
            self.clicks_limit,
            10,
            10_000,
            self.impressions_limit,
            100,
            100_000,
        );
        let (start_date, end_date) =
            generate_validated_option(self.start_date, 1, 100, self.end_date, 1, 100);

        CampaignUpdate {
            impressions_limit,
            clicks_limit,
            cost_per_impression: self
                .cost_per_impression
                .unwrap_or_else(|| return_randomly(|| rng.random_range(0.1..10.0)))
                .map(|c| c.round_to_digits(2)),
            cost_per_click: self
                .cost_per_click
                .unwrap_or_else(|| return_randomly(|| rng.random_range(1.0..100.0)))
                .map(|c| c.round_to_digits(2)),
            ad_title: self
                .ad_title
                .unwrap_or_else(|| return_randomly(|| format!("Updated Campaign_{}", counter))),
            ad_text: self.ad_text.unwrap_or_else(|| {
                return_randomly(|| format!("Updated content for Campaign_{}", counter))
            }),
            start_date,
            end_date,
            targeting: self
                .targeting
                .unwrap_or_else(|| return_randomly(Targeting::new)),
        }
    }
}
impl CampaignUpdateBuilder {
    pub fn with_impressions_limit(mut self, impressions_limit: Option<i64>) -> Self {
        self.impressions_limit = Some(impressions_limit);
        self
    }

    pub fn with_clicks_limit(mut self, clicks_limit: Option<i64>) -> Self {
        self.clicks_limit = Some(clicks_limit);
        self
    }

    pub fn with_cost_per_impression(mut self, cost_per_impression: Option<f64>) -> Self {
        self.cost_per_impression = Some(cost_per_impression);
        self
    }

    pub fn with_cost_per_click(mut self, cost_per_click: Option<f64>) -> Self {
        self.cost_per_click = Some(cost_per_click);
        self
    }

    pub fn with_ad_title(mut self, ad_title: Option<impl Into<String>>) -> Self {
        self.ad_title = Some(ad_title.map(|t| t.into()));
        self
    }

    pub fn with_ad_text(mut self, ad_text: Option<impl Into<String>>) -> Self {
        self.ad_text = Some(ad_text.map(|t| t.into()));
        self
    }

    pub fn with_start_date(mut self, start_date: Option<i64>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: Option<i64>) -> Self {
        self.end_date = Some(end_date);
        self
    }

    pub fn with_targeting(mut self, targeting: Option<Targeting>) -> Self {
        self.targeting = Some(targeting);
        self
    }
}

pub struct TargetingBuilder {
    gender: Option<Option<String>>,
    age_from: Option<Option<i64>>,
    age_to: Option<Option<i64>>,
    location: Option<Option<String>>,
}
impl Builder<Targeting> for TargetingBuilder {
    fn new() -> Self {
        Self {
            gender: None,
            age_from: None,
            age_to: None,
            location: None,
        }
    }

    fn build(self) -> Targeting {
        let mut rng = rng();
        let (age_from, age_to) =
            generate_validated_option(self.age_from, 13, 80, self.age_to, 13, 80);

        Targeting {
            gender: self.gender.unwrap_or_else(|| match rng.random_range(0..4) {
                0 => Some(TargetingGender::MALE.to_string()),
                1 => Some(TargetingGender::FEMALE.to_string()),
                2 => Some(TargetingGender::ALL.to_string()),
                _ => None,
            }),
            age_from,
            age_to,
            location: self
                .location
                .unwrap_or_else(|| return_randomly(|| generate_location(LocationType::City))),
        }
    }
}
impl TargetingBuilder {
    pub fn with_gender(mut self, gender: Option<&str>) -> Self {
        self.gender = Some(gender.map(|g| g.to_string()));
        self
    }

    pub fn with_age_from(mut self, age_from: Option<i64>) -> Self {
        self.age_from = Some(age_from);
        self
    }

    pub fn with_age_to(mut self, age_to: Option<i64>) -> Self {
        self.age_to = Some(age_to);
        self
    }

    pub fn with_location(mut self, location: Option<&str>) -> Self {
        self.location = Some(location.map(|l| l.into()));
        self
    }
}

pub struct ClickInfoBuilder {
    pub client_id: Option<Uuid>,
}
impl Builder<ClickInfo> for ClickInfoBuilder {
    fn new() -> Self {
        Self { client_id: None }
    }

    fn build(self) -> ClickInfo {
        ClickInfo {
            client_id: self.client_id.unwrap_or_else(|| Uuid::now_v7()),
        }
    }
}
impl ClickInfoBuilder {
    pub fn with_client_id(mut self, client_id: Uuid) -> Self {
        self.client_id = Some(client_id);
        self
    }
}

pub struct StatsBuilder {
    pub campaign: Option<Campaign>,
    pub impressions_count: Option<i64>,
    pub clicks_count: Option<i64>,
}
impl Builder<Stats> for StatsBuilder {
    fn new() -> Self {
        Self {
            campaign: None,
            impressions_count: None,
            clicks_count: None,
        }
    }

    fn build(self) -> Stats {
        let campaign = CreateCampaign::new();
        let (clicks_count, impressions_count) = generate_validated(
            self.clicks_count,
            10,
            campaign.clicks_limit,
            self.impressions_count,
            100,
            campaign.impressions_limit,
        );
        let spent_impressions = impressions_count as f64 * campaign.cost_per_impression;
        let spent_clicks = clicks_count as f64 * campaign.cost_per_click;

        Stats {
            impressions_count,
            clicks_count,
            conversion: (clicks_count as f64 / impressions_count as f64) as f32 * 100.0,
            spent_impressions,
            spent_clicks,
            spent_total: spent_impressions + spent_clicks,
        }
    }
}

fn generate_validated<T: Ord + SampleUniform + Copy>(
    from: Option<T>,
    min_from: T,
    max_from: T,
    to: Option<T>,
    min_to: T,
    max_to: T,
) -> (T, T) {
    let mut rng = rng();
    match (from, to) {
        (Some(f), Some(t)) => (f, t),
        (Some(f), None) => (f, rng.random_range(min_to.max(f)..=max_to.max(f))),
        (None, Some(t)) => (rng.random_range(min_from.min(t)..=max_from.min(t)), t),
        (None, None) => {
            let from = rng.random_range(min_from..=max_from);
            (from, rng.random_range(min_to.max(from)..=max_to))
        }
    }
}

fn generate_validated_option<T: Ord + SampleUniform + Copy + Default>(
    from: Option<Option<T>>,
    min_from: T,
    max_from: T,
    to: Option<Option<T>>,
    min_to: T,
    max_to: T,
) -> (Option<T>, Option<T>) {
    let mut rng = rng();
    match (from, to) {
        (Some(v1), Some(v2)) => (v1, v2),
        (Some(None), None) => (None, return_randomly(|| rng.random_range(min_to..=max_to))),
        (None, Some(None)) => (
            return_randomly(|| rng.random_range(min_from..=max_from)),
            None,
        ),
        (Some(Some(f)), None) => (
            Some(f),
            return_randomly(|| rng.random_range(min_to.max(f)..=max_to.max(f))),
        ),
        (None, Some(Some(f))) => (
            return_randomly(|| rng.random_range(min_from.min(f)..=max_from.min(f))),
            Some(f),
        ),
        (None, None) => {
            let from = return_randomly(|| rng.random_range(min_from..=max_from));
            (
                from,
                return_randomly(|| rng.random_range(min_to.max(from.unwrap_or_default())..=max_to)),
            )
        }
    }
}

fn return_randomly<T>(value_gen: impl FnOnce() -> T) -> Option<T> {
    if random_bool(0.5) {
        Some(value_gen())
    } else {
        None
    }
}

pub enum LocationType {
    Country,
    City,
    MetroStation,
}

pub fn generate_location(l_type: LocationType) -> String {
    let mut rng = rng();

    match l_type {
        LocationType::Country => {
            let countries = [
                "United States",
                "Canada",
                "Mexico",
                "Brazil",
                "Argentina",
                "UK",
                "France",
                "Germany",
                "Italy",
                "Spain",
                "Russia",
                "China",
                "Japan",
                "South Korea",
                "India",
                "Australia",
                "Egypt",
                "South Africa",
                "Nigeria",
                "Kenya",
            ];
            countries.choose(&mut rng).unwrap().to_string()
        }
        LocationType::City => {
            let cities = [
                ("New York", "US"),
                ("Los Angeles", "US"),
                ("Chicago", "US"),
                ("Toronto", "CA"),
                ("Vancouver", "CA"),
                ("Montreal", "CA"),
                ("London", "UK"),
                ("Manchester", "UK"),
                ("Liverpool", "UK"),
                ("Paris", "FR"),
                ("Lyon", "FR"),
                ("Marseille", "FR"),
                ("Berlin", "DE"),
                ("Munich", "DE"),
                ("Hamburg", "DE"),
                ("Tokyo", "JP"),
                ("Osaka", "JP"),
                ("Kyoto", "JP"),
                ("Mumbai", "IN"),
                ("Delhi", "IN"),
                ("Bangalore", "IN"),
                ("Sydney", "AU"),
                ("Melbourne", "AU"),
                ("Brisbane", "AU"),
            ];
            let (city, country) = cities.choose(&mut rng).unwrap();
            format!("{}, {}", city, country)
        }
        LocationType::MetroStation => {
            let stations = [
                ("Times Square", "New York"),
                ("Grand Central", "New York"),
                ("Shinjuku", "Tokyo"),
                ("Shibuya", "Tokyo"),
                ("Gare du Nord", "Paris"),
                ("Châtelet", "Paris"),
                ("Alexanderplatz", "Berlin"),
                ("Hauptbahnhof", "Berlin"),
                ("Oxford Circus", "London"),
                ("Victoria Station", "London"),
                ("Union Station", "Toronto"),
                ("Central Station", "Sydney"),
            ];
            let (station, city) = stations.choose(&mut rng).unwrap();
            format!("{} Station, {}", station, city)
        }
    }
}
