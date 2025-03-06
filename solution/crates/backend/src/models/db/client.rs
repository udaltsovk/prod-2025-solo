use serde::{Deserialize, Serialize};
use surrealdb::{RecordId, Result};
use surrealdb_helper::SurrealDB;
use uuid::Uuid;

use crate::models::{dto::Client, Gender};

use super::DBModel;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBClient {
    pub id: RecordId,
    pub login: String,
    pub age: i32,
    pub location: String,
    pub gender: Gender,
}

impl DBModel for DBClient {
    const TABLE: &str = "client";
}

impl DBClient {
    pub async fn get(id: Uuid, db: &SurrealDB) -> Result<Option<Self>> {
        Ok(db.0.select(Self::record_id_from_uuid(&id)).await?)
    }

    pub async fn bulk(clients: Vec<Client>, db: &SurrealDB) -> Result<Vec<Self>> {
        Ok(db
            .0
            .query("BEGIN")
            .query(
                r#"
                    FOR $client IN $clients {
                        UPSERT type::table($client_table) CONTENT $client;
                    }
                "#,
            )
            .query(
                r#"
                    SELECT * FROM type::table($client_table)
                        WHERE id ∈ $clients.map(|$client| $client.id)
                "#,
            )
            .bind((
                "clients",
                clients.iter().map(Self::from).collect::<Vec<Self>>(),
            ))
            .bind(("client_table", Self::TABLE))
            .query("COMMIT")
            .await?
            .take(1)?)
    }
}

impl From<&Client> for DBClient {
    fn from(dto: &Client) -> Self {
        Self {
            id: Self::record_id_from_uuid(&dto.id),
            login: dto.login.clone(),
            age: dto.age,
            location: dto.location.clone(),
            gender: dto.gender.clone(),
        }
    }
}
