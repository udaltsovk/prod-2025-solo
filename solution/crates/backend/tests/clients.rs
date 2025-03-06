mod common;

use actix_http::StatusCode;
use common::{
    environment::with_test_environment,
    models::{
        builders::{Buildable, Builder},
        Client, Model,
    },
};
use rand::{rng, seq::IndexedRandom};
use rstest::rstest;
use uuid::Uuid;
use validator::ValidateLength;

#[rstest]
#[case::gender(
    Client::builder()
        .with_gender("baobab")
        .build()
)]
#[case::age(
    Client::builder()
        .with_age(-1)
        .build()
)]
#[case::profanity_login(
    Client::builder()
        .with_login("питон")
        .build()
)]
#[case::profanity_location(
    Client::builder()
        .with_location("питон")
        .build()
)]
#[actix_rt::test]
async fn clients_upsertion_invalid(#[case] client: Client) {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let clients = vec![client];

        let resp = api.upsert_clients(clients).await;
        assert_status!(resp, StatusCode::BAD_REQUEST);
    })
    .await
}

#[actix_rt::test]
async fn clients_upsertion() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let clients: Vec<Client> = (0..100).map(|_| Client::new()).collect();

        let resp = api.upsert_clients(clients.clone()).await;
        assert_json!(resp, StatusCode::CREATED, Vec<Client>, clients);
    })
    .await
}

#[actix_rt::test]
async fn clients_upsertion_with_duplicates() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let mut client = Client::new();
        let clients: Vec<Client> = (0..100)
            .map(|i| {
                client.age = i;
                client.clone()
            })
            .collect();

        let resp = api.upsert_clients(clients).await;
        assert_status!(resp, StatusCode::CREATED);

        let clients: Vec<Client> = Vec::from_resp(resp).await;
        assert_eq!(clients.length(), Some(1));
    })
    .await
}

#[actix_rt::test]
async fn get_client_by_id_non_existent() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let resp = api.get_client_by_id(Uuid::now_v7()).await;
        assert_status!(resp, StatusCode::NOT_FOUND);
    })
    .await
}

#[actix_rt::test]
async fn get_client_by_id() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let clients: Vec<Client> = (0..100).map(|_| Client::new()).collect();
        let client = clients.choose(&mut rng()).unwrap().clone();

        let resp = api.upsert_clients(clients).await;
        assert_status!(resp, StatusCode::CREATED);

        let resp = api.get_client_by_id(client.id).await;
        assert_json!(resp, StatusCode::OK, Client, client);
    })
    .await
}
