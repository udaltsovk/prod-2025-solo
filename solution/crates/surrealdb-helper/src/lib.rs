use include_dir::Dir;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};
use surrealdb_migrations::MigrationRunner;

#[derive(Clone)]
pub struct SurrealDB(pub Surreal<Client>);

impl SurrealDB {
    pub async fn init(
        address: &str,
        namespace: &str,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        let surreal: Surreal<Client> = Surreal::init();

        surreal.connect::<Ws>(address).await?;
        surreal.signin(Root { username, password }).await?;

        surreal.use_ns(namespace).use_db(database).await?;

        Ok(Self(surreal))
    }

    pub async fn migrate(self, migrations_dir: &Dir<'static>) -> Self {
        MigrationRunner::new(&self.0)
            .load_files(migrations_dir)
            .up()
            .await
            .expect("Failed to run migrations");

        self
    }
}
