mod common;

use actix_http::StatusCode;
use common::{environment::with_test_environment, models::Time};
use rstest::rstest;
use serial_test::serial;

#[rstest]
#[serial(time)]
#[actix_rt::test]
pub async fn day_advancing() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let mut time = Time::start();

        for _ in 1..=100 {
            let resp = api.advance_day(time).await;
            assert_json!(resp, StatusCode::OK, Time, time);
            time.increase_day();
        }
    })
    .await
}
// TODO: написать больше тестов на обновление дня

#[rstest]
#[serial(time)]
#[actix_rt::test]
pub async fn advance_day_back() {
    with_test_environment(|test_env| async move {
        let api = test_env.api;

        let mut time = Time::start();

        let resp = api.advance_day(time).await;
        assert_status!(&resp, StatusCode::OK);

        time.increase_day();

        let resp = api.advance_day(time).await;
        assert_status!(&resp, StatusCode::OK);

        time.decrease_day();

        let resp = api.advance_day(time).await;
        assert_status!(&resp, StatusCode::BAD_REQUEST);
    })
    .await
}
