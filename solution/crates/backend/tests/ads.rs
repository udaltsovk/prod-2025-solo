use actix_http::StatusCode;
use common::{
    environment::with_test_environment,
    models::{
        builders::{Buildable, Builder},
        Advertiser, Campaign, ClickInfo, Client, CreateCampaign, MLScore, Model, Targeting,
    },
};
use rand::{random_bool, rng, seq::IndexedRandom, Rng};
use uuid::Uuid;

mod common;

#[actix_rt::test]
async fn get_ad_for_client_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api.get_ad_for_client(Uuid::now_v7()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[ignore]
#[actix_rt::test]
async fn dev() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let clients: Vec<Client> = (0..100).map(|_| Client::new()).collect();
        let client = clients.choose(&mut rng()).unwrap().clone();
        let resp = api.upsert_clients(clients.clone()).await;
        assert_status!(resp, StatusCode::CREATED);

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        for client in clients {
            let ml_score = MLScore::builder()
                .with_advertiser_id(advertiser.id)
                .with_client_id(client.id)
                .build();
            let resp = api.upsert_ml_score(ml_score).await;
            assert_status!(resp, StatusCode::OK);
        }

        let campaigns: Vec<CreateCampaign> = (0..100)
            .map(|_| {
                CreateCampaign::builder()
                    .with_start_date(rng().random_range(1..=4))
                    .with_targeting(if random_bool(0.33) {
                        Targeting::builder()
                            .with_age_to(Some(client.age))
                            .with_gender(Some(&client.gender))
                            .with_location(Some(&client.location))
                            .build()
                    } else {
                        Targeting::new()
                    })
                    .build()
            })
            .collect();

        for c in campaigns {
            let resp = api.create_campaign(advertiser.id, c).await;
            assert_status!(resp, StatusCode::CREATED);
        }
        panic!("I need the db")
    })
    .await
}

// TODO: написать больше тестов на получение рекламных объявлений

#[actix_rt::test]
async fn record_ad_click_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = CreateCampaign::new();
        let resp = api.create_campaign(advertiser.id, campaign.clone()).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;

        let resp = api
            .record_ad_click(campaign_resp.id, ClickInfo::new())
            .await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[actix_rt::test]
async fn record_ad_click_not_viewed() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let client = Client::new();
        let resp = api.upsert_clients(vec![client.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = CreateCampaign::new();
        let resp = api.create_campaign(advertiser.id, campaign.clone()).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;
        let click_info = ClickInfo::builder().with_client_id(client.id).build();

        let resp = api.record_ad_click(campaign_resp.id, click_info).await;
        assert_status!(resp, StatusCode::CONFLICT);
    })
    .await
}

// TODO: написать больше тестов на переходы по рекламным объявлениям
