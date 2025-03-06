use backend::{config, utils::minio::Minio};
use rand::random;

#[derive(Clone)]
pub struct TemporaryFileHost {
    pub minio: Minio,
    bucket_name: String,
}

impl TemporaryFileHost {
    pub async fn create() -> Self {
        let bucket_name = generate_random_name(&config::MINIO_BUCKET);

        let minio = Minio::init(
            &config::MINIO_BASE_URL,
            &config::MINIO_USER,
            &config::MINIO_PASSWORD,
            &config::MINIO_BUCKET,
        )
        .await
        .expect("Failed to init the file host");

        Self { minio, bucket_name }
    }

    pub async fn cleanup(self) {
        self.minio
            .remove_bucket()
            .await
            .expect("File host bucket deletion failed");
    }
}

pub fn generate_random_name(str: &str) -> String {
    let mut str = String::from(str);
    str.push_str(&random::<u64>().to_string()[..8]);
    str
}
