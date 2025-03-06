use actix_multipart::form::tempfile::TempFile;
use actix_web::http::StatusCode;
use bytes::Bytes;
use minio::s3::{
    args::{
        BucketExistsArgs, GetObjectArgs, MakeBucketArgs, PutObjectArgs, RemoveBucketArgs,
        RemoveObjectArgs,
    },
    client::{Client, ClientBuilder},
    creds::StaticProvider,
    error::Error,
    http::BaseUrl,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Minio {
    client: Client,
    bucket_name: String,
}

impl Minio {
    pub async fn init(
        base_url: &str,
        user: &str,
        password: &str,
        bucket_name: &str,
    ) -> Result<Self, Error> {
        log::info!("Trying to connect to MinIO at `{:?}`", base_url);
        let base_url = base_url.parse::<BaseUrl>()?;

        let client = ClientBuilder::new(base_url.clone())
            .provider(Some(Box::new(StaticProvider::new(user, password, None))))
            .build()?;

        let exists: bool = client
            .bucket_exists(&BucketExistsArgs::new(bucket_name).unwrap())
            .await?;

        if !exists {
            client
                .make_bucket(&MakeBucketArgs::new(bucket_name).unwrap())
                .await
                .unwrap();
        };

        Ok(Self {
            client,
            bucket_name: bucket_name.into(),
        })
    }

    pub async fn put_image(
        &self,
        advertiser_id: &Uuid,
        campaign_id: &Uuid,
        file: TempFile,
    ) -> Result<(), Error> {
        let file_size = Some(file.size);
        let file_name = Self::ids_to_filename(advertiser_id, campaign_id);
        let mime_type = file.content_type.unwrap().to_string();
        let mut file = file.file;

        let mut args =
            PutObjectArgs::new(&self.bucket_name, &file_name, &mut file, file_size, None)?;
        args.content_type = &mime_type;

        self.client.put_object(&mut args).await?;

        Ok(())
    }

    pub async fn get_image(
        &self,
        advertiser_id: &Uuid,
        campaign_id: &Uuid,
    ) -> Result<Option<(String, Bytes)>, Error> {
        let resp = if let Ok(resp) = self
            .client
            .get_object(&GetObjectArgs::new(
                &self.bucket_name,
                &Self::ids_to_filename(advertiser_id, campaign_id),
            )?)
            .await
        {
            resp
        } else {
            return Ok(None);
        };

        Ok(if resp.status() == StatusCode::OK {
            Some((
                resp.headers()
                    .get("content-type")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                resp.bytes().await?,
            ))
        } else {
            None
        })
    }

    pub async fn remove_image(
        &self,
        advertiser_id: &Uuid,
        campaign_id: &Uuid,
    ) -> Result<(), Error> {
        self.client
            .remove_object(&RemoveObjectArgs::new(
                &self.bucket_name,
                &Self::ids_to_filename(advertiser_id, campaign_id),
            )?)
            .await?;

        Ok(())
    }

    pub async fn remove_bucket(&self) -> Result<(), Error> {
        self.client
            .remove_bucket(&RemoveBucketArgs::new(&self.bucket_name)?)
            .await?;
        Ok(())
    }

    fn ids_to_filename(advertiser_id: &Uuid, campaign_id: &Uuid) -> String {
        format!("{advertiser_id}_{campaign_id}")
    }
}
