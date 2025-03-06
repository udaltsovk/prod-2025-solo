use backend::config;
use include_dir::include_dir;
use rand::random;
use surrealdb_helper::SurrealDB;

#[derive(Clone)]
pub struct TemporaryDatabase {
    pub surreal: SurrealDB,
    database_name: String,
}

impl TemporaryDatabase {
    pub async fn create() -> Self {
        let database_name = generate_random_name(&config::DB_NAME);

        let surreal = SurrealDB::init(
            &config::DB_ADDRESS,
            &config::DB_NAMESPACE,
            &database_name,
            &config::DB_USER,
            &config::DB_PASSWORD,
        )
        .await
        .expect("Failed to init the test database")
        .migrate(&include_dir!("crates/backend/db"))
        .await;

        Self {
            surreal,
            database_name,
        }
    }

    pub async fn cleanup(self) {
        self.surreal
            .0
            .query(format!("REMOVE DATABASE {}", &self.database_name))
            .await
            .expect("Database deletion failed");
    }
}

pub fn generate_random_name(str: &str) -> String {
    let mut str = String::from(str);
    str.push_str(&random::<u64>().to_string()[..8]);
    str
}
