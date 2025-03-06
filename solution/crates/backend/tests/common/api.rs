use std::rc::Rc;

use actix_web::{
    dev::ServiceResponse,
    test::{self, TestRequest},
    App,
};
use backend::BackendConfig;
use utoipa_actix_web::AppExt;

use super::{
    environment::LocalService,
    models::{Advertiser, CampaignUpdate, ClickInfo, Client, CreateCampaign, MLScore, Time},
};

#[derive(Clone)]
pub struct Api {
    pub test_app: Rc<dyn LocalService>,
}

#[allow(dead_code)]
impl Api {
    pub async fn build(solution_config: BackendConfig) -> Self {
        let app = App::new()
            .into_utoipa_app()
            .configure(solution_config.clone().build())
            .into_app();

        let test_app = Rc::new(test::init_service(app).await);

        Self { test_app }
    }

    pub async fn call(&self, req: actix_http::Request) -> ServiceResponse {
        self.test_app.call(req).await.unwrap()
    }

    pub async fn upsert_clients(&self, clients: Vec<Client>) -> ServiceResponse {
        let req = TestRequest::post()
            .uri("/clients/bulk")
            .set_json(clients)
            .to_request();
        self.call(req).await
    }

    pub async fn get_client_by_id(&self, client_id: impl Into<String>) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!("/clients/{}", client_id.into()))
            .to_request();
        self.call(req).await
    }

    pub async fn upsert_advertisers(&self, advertisers: Vec<Advertiser>) -> ServiceResponse {
        let req = TestRequest::post()
            .uri("/advertisers/bulk")
            .set_json(advertisers)
            .to_request();
        self.call(req).await
    }

    pub async fn get_advertiser_by_id(&self, advertiser_id: impl Into<String>) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!("/advertisers/{}", advertiser_id.into()))
            .to_request();
        self.call(req).await
    }

    pub async fn upsert_ml_score(&self, ml_score: MLScore) -> ServiceResponse {
        let req = TestRequest::post()
            .uri("/ml-scores")
            .set_json(ml_score)
            .to_request();
        self.call(req).await
    }

    pub async fn create_campaign(
        &self,
        advertiser_id: impl Into<String>,
        campaign: CreateCampaign,
    ) -> ServiceResponse {
        let req = TestRequest::post()
            .uri(&format!("/advertisers/{}/campaigns", advertiser_id.into()))
            .set_json(campaign)
            .to_request();
        self.call(req).await
    }

    pub async fn list_campaigns(
        &self,
        advertiser_id: impl Into<String>,
        size: Option<i64>,
        page: Option<i64>,
    ) -> ServiceResponse {
        let query_params: &str = match (size, page) {
            (None, None) => "",
            (Some(size), None) => &format!("?size={size}"),
            (None, Some(page)) => &format!("?page={page}"),
            (Some(size), Some(page)) => &format!("?size={size}&page={page}"),
        };

        let req = TestRequest::get()
            .uri(&format!(
                "/advertisers/{}/campaigns{query_params}",
                advertiser_id.into()
            ))
            .to_request();

        self.call(req).await
    }

    pub async fn get_campaign_by_id(
        &self,
        advertiser_id: impl Into<String>,
        campaign_id: impl Into<String>,
    ) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!(
                "/advertisers/{}/campaigns/{}",
                advertiser_id.into(),
                campaign_id.into()
            ))
            .to_request();
        self.call(req).await
    }

    pub async fn update_campaign(
        &self,
        advertiser_id: impl Into<String>,
        campaign_id: impl Into<String>,
        campaign_update: CampaignUpdate,
    ) -> ServiceResponse {
        let req = TestRequest::put()
            .uri(&format!(
                "/advertisers/{}/campaigns/{}",
                advertiser_id.into(),
                campaign_id.into()
            ))
            .set_json(campaign_update)
            .to_request();
        self.call(req).await
    }

    pub async fn delete_campaign(
        &self,
        advertiser_id: impl Into<String>,
        campaign_id: impl Into<String>,
    ) -> ServiceResponse {
        let req = TestRequest::delete()
            .uri(&format!(
                "/advertisers/{}/campaigns/{}",
                advertiser_id.into(),
                campaign_id.into()
            ))
            .to_request();
        self.call(req).await
    }

    pub async fn get_ad_for_client(&self, client_id: impl Into<String>) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!("/ads?client_id={}", client_id.into()))
            .to_request();
        self.call(req).await
    }

    pub async fn record_ad_click(
        &self,
        ad_id: impl Into<String>,
        click_info: ClickInfo,
    ) -> ServiceResponse {
        let req = TestRequest::post()
            .uri(&format!("/ads/{}/click", ad_id.into()))
            .set_json(click_info)
            .to_request();
        self.call(req).await
    }

    pub async fn get_campaign_stats(&self, campaign_id: impl Into<String>) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!("/statistics/campaigns/{}", campaign_id.into()))
            .to_request();
        self.call(req).await
    }

    pub async fn get_advertiser_campaigns_stats(
        &self,
        advertiser_id: impl Into<String>,
    ) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!("/statistics/advertisers/{}", advertiser_id.into()))
            .to_request();
        self.call(req).await
    }

    pub async fn get_campaign_daily_stats(
        &self,
        campaign_id: impl Into<String>,
    ) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!(
                "/statistics/campaigns/{}/daily",
                campaign_id.into()
            ))
            .to_request();
        self.call(req).await
    }

    pub async fn get_advertiser_daily_stats(
        &self,
        advertiser_id: impl Into<String>,
    ) -> ServiceResponse {
        let req = TestRequest::get()
            .uri(&format!(
                "/statistics/advertisers/{}/daily",
                advertiser_id.into()
            ))
            .to_request();
        self.call(req).await
    }

    pub async fn advance_day(&self, new_time: Time) -> ServiceResponse {
        let req = TestRequest::post()
            .uri("/time/advance")
            .set_json(new_time)
            .to_request();
        self.call(req).await
    }
}
