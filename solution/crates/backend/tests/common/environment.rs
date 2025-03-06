use std::future::Future;

use actix_web::dev::ServiceResponse;
use backend::{app_setup, models::dto::Time};

use super::{api::Api, database::TemporaryDatabase, file_host::TemporaryFileHost};

pub async fn with_test_environment<Fut>(f: impl FnOnce(TestEnvironment) -> Fut)
where
    Fut: Future<Output = ()>,
{
    let test_env = TestEnvironment::build().await;
    let db = test_env.db.clone();
    let file_host = test_env.file_host.clone();
    Time::cleanup();
    f(test_env).await;
    file_host.cleanup().await;
    db.cleanup().await;
}

pub struct TestEnvironment {
    pub api: Api,
    pub db: TemporaryDatabase,
    pub file_host: TemporaryFileHost,
}

impl TestEnvironment {
    pub async fn build() -> Self {
        let db = TemporaryDatabase::create().await;
        let file_host = TemporaryFileHost::create().await;
        let config = app_setup(db.surreal.clone(), file_host.minio.clone());

        let api = Api::build(config).await;

        Self { api, db, file_host }
    }
}

pub trait LocalService {
    fn call(
        &self,
        req: actix_http::Request,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<ServiceResponse, actix_web::Error>>>,
    >;
}
impl<S> LocalService for S
where
    S: actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    fn call(
        &self,
        req: actix_http::Request,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<ServiceResponse, actix_web::Error>>>,
    > {
        Box::pin(self.call(req))
    }
}
