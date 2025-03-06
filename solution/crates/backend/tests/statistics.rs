use actix_http::StatusCode;
use common::{
    environment::with_test_environment,
    models::{
        builders::{Buildable, Builder},
        Advertiser, Campaign, CreateCampaign, Model, OptionalModel, Stats, Time,
    },
};
use rstest::rstest;
use serial_test::serial;
use uuid::Uuid;

mod common;

#[rstest]
#[actix_rt::test]
async fn get_campaign_stats_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api.get_campaign_stats(Uuid::now_v7()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[rstest]
#[actix_rt::test]
async fn get_campaign_stats_empty() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = CreateCampaign::new();

        let resp = api.create_campaign(advertiser.id, campaign).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;

        let resp = api.get_campaign_stats(campaign_resp.id).await;
        assert_json!(resp, StatusCode::OK, Stats, Stats::empty());
    })
    .await
}

#[rstest]
#[actix_rt::test]
async fn get_campaign_stats_deletion() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = CreateCampaign::new();

        let resp = api.create_campaign(advertiser.id, campaign).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;

        let resp = api.get_campaign_stats(campaign_resp.id).await;
        assert_json!(resp, StatusCode::OK, Stats, Stats::empty());

        let resp = api.delete_campaign(advertiser.id, campaign_resp.id).await;
        assert_status!(resp, StatusCode::NO_CONTENT);

        let resp = api.get_campaign_stats(campaign_resp.id).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}
// TODO: написать больше тестов на получение статистики по рекламной кампании

#[rstest]
#[actix_rt::test]
async fn get_advertiser_campaigns_stats_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api.get_advertiser_campaigns_stats(Uuid::now_v7()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[rstest]
#[actix_rt::test]
async fn get_advertiser_campaigns_stats_empty() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api.get_advertiser_campaigns_stats(advertiser.id).await;
        assert_json!(resp, StatusCode::OK, Stats, Stats::empty());
    })
    .await
}
// TODO: написать больше тестов на получение статистики по рекламодателю

#[rstest]
#[actix_rt::test]
async fn get_campaign_daily_stats_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api.get_campaign_daily_stats(Uuid::now_v7()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[rstest]
#[serial(time)]
#[actix_rt::test]
async fn get_campaign_daily_stats_empty() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let mut time = Time::start();

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign = CreateCampaign::builder()
            .with_start_date(1)
            .with_end_date(100)
            .build();

        let resp = api.create_campaign(advertiser.id, campaign).await;
        assert_status!(resp, StatusCode::CREATED);

        let campaign_resp = Campaign::from_resp(resp).await;
        let mut stats: Vec<Stats> = vec![];

        for day in 1..=campaign_resp.inner.end_date {
            let resp = api.advance_day(time).await;
            assert_status!(resp, StatusCode::OK);

            if campaign_resp.inner.start_date <= day && day <= campaign_resp.inner.end_date {
                stats.push(Stats::empty());
            }
            let resp = api.get_campaign_daily_stats(campaign_resp.id).await;
            assert_json!(resp, StatusCode::OK, Vec<Stats>, stats.clone());

            time.increase_day();
        }
    })
    .await
}
// TODO: написать больше тестов на получение ежедневной статистики по рекламной кампании

#[rstest]
#[actix_rt::test]
async fn get_advertiser_daily_stats_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api.get_advertiser_daily_stats(Uuid::now_v7()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[rstest]
#[serial(time)]
#[actix_rt::test]
async fn get_advertiser_daily_stats_empty() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let mut time = Time::start();

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let mut stats: Vec<Stats> = vec![];

        for _ in 0..100 {
            stats.push(Stats::empty());

            let resp = api.get_advertiser_daily_stats(advertiser.id).await;
            assert_json!(resp, StatusCode::OK, Vec<Stats>, stats.clone());

            time.increase_day();
            let resp = api.advance_day(time).await;
            assert_status!(resp, StatusCode::OK);
        }
    })
    .await
}
// TODO: написать больше тестов на получение ежедневной статистики по рекламодателю
