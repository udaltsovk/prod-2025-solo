mod common;

use actix_http::StatusCode;
use common::{environment::with_test_environment, models::{builders::{Buildable, Builder}, Advertiser, Client, MLScore, Model}
};
use rand::{rng, seq::IndexedRandom};
use rstest::rstest;
use uuid::Uuid;
use validator::ValidateLength;

#[rstest]
#[case::profanity_name(
    Advertiser::builder()
        .with_name("питон")
        .build()
)]
#[actix_rt::test]
async fn advertisers_upsertion_invalid(#[case] advertiser: Advertiser) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertisers = vec![advertiser];

        let resp = api.upsert_advertisers(advertisers).await;
        assert_status!(resp, StatusCode::BAD_REQUEST);
    })
    .await
}

#[actix_rt::test]
async fn advertisers_upsertion() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertisers: Vec<Advertiser> = (0..100).map(|_| Advertiser::new()).collect();

        let resp = api.upsert_advertisers(advertisers.clone()).await;
        assert_json!(resp, StatusCode::CREATED, Vec<Advertiser>, advertisers);
    })
    .await
}

#[actix_rt::test]
async fn advertisers_upsertion_with_duplicates() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let mut advertiser = Advertiser::new();
        let advertisers: Vec<Advertiser> = (0..100)
            .map(|i| {
                advertiser.name = i.to_string();
                advertiser.clone()
            })
            .collect();

        let resp = api.upsert_advertisers(advertisers).await;
        assert_status!(resp, StatusCode::CREATED);

        let advertisers: Vec<Advertiser> = Vec::from_resp(resp).await;
        assert_eq!(advertisers.length(), Some(1));
    })
    .await
}

#[actix_rt::test]
async fn get_advertiser_by_id_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api.get_advertiser_by_id(Uuid::now_v7()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[actix_rt::test]
async fn get_advertiser_by_id() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let advertisers: Vec<Advertiser> = (0..100).map(|_| Advertiser::new()).collect();
        let advertiser = advertisers.choose(&mut rng()).unwrap().clone();

        let resp = api.upsert_advertisers(advertisers).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api.get_advertiser_by_id(advertiser.id).await;
        assert_json!(resp, StatusCode::OK, Advertiser, advertiser);
    })
    .await
}

#[rstest]
#[case::score(|advertiser_id, client_id| 
    MLScore::builder()
        .with_advertiser_id(advertiser_id)
        .with_client_id(client_id)
        .with_score(-1)
        .build()
)]
#[actix_rt::test]
async fn upsert_ml_score_invalid(#[case] ml_score: impl FnOnce(Uuid, Uuid) -> MLScore) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let client = Client::new();
        let resp = api.upsert_clients(vec![client.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api
            .upsert_ml_score(ml_score(advertiser.id, client.id))
            .await;
        assert_status!(resp, StatusCode::BAD_REQUEST);
    })
    .await
}

#[actix_rt::test]
async fn upsert_ml_score_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let mut ml_score = MLScore::new();

        let resp = api.upsert_ml_score(ml_score.clone()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        ml_score.advertiser_id = advertiser.id;

        let resp = api.upsert_ml_score(ml_score).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[actix_rt::test]
async fn upsert_ml_score() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let client = Client::new();
        let resp = api.upsert_clients(vec![client.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let advertiser = Advertiser::new();
        let resp = api.upsert_advertisers(vec![advertiser.clone()]).await;
        assert_status!(resp, StatusCode::CREATED);

        let ml_score = MLScore::builder()
            .with_advertiser_id(advertiser.id)
            .with_client_id(client.id)
            .build();

        let resp = api.upsert_ml_score(ml_score).await;
        assert_status!(resp, StatusCode::OK);
    })
    .await
}
