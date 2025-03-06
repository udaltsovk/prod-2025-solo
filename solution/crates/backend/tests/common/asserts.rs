#[macro_export]
macro_rules! assert_status {
    ($response:expr, $status:expr) => {
        let status = $response.status();
        assert_eq!(status, $status, "{:#?}", $response.response().body());
    };
}

#[macro_export]
macro_rules! assert_json {
    ($response:expr, $status:expr, $model:ty, $object:expr) => {
        assert_status!($response, $status);
        let object_got = <$model as crate::common::models::Model>::try_from_resp($response)
            .await
            .unwrap();
        assert_eq!(object_got, $object, "JSON mismatch");
    };
}
