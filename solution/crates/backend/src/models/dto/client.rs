use serde::{Deserialize, Serialize};
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{
        db::{DBClient, DBModel},
        Gender,
    },
    routes::ApiError,
    utils::validation::check_profanity,
};

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug)]
#[schema(description = "Объект, представляющий клиента системы.")]
pub struct Client {
    #[serde(rename = "client_id")]
    /// Уникальный идентификатор клиента (UUID).
    pub id: Uuid,

    #[validate(custom(function = "check_profanity"))]
    /// Логин клиента в системе.
    pub login: String,

    #[schema(examples(0), minimum = 0)]
    #[validate(range(min = 0))]
    /// Возраст клиента.
    pub age: i32,

    #[validate(custom(function = "check_profanity"))]
    /// Локация клиента (город, регион или район).
    pub location: String,

    /// Пол клиента (MALE или FEMALE).
    pub gender: Gender,
}

impl Client {
    pub async fn get_by_id(client_id: Uuid, db: &SurrealDB) -> Result<Self, ApiError> {
        match DBClient::get(client_id, db).await? {
            None => Err(ApiError::NotFound(format!(
                "Client with UUID `{}`",
                client_id
            ))),
            Some(client) => Ok((&client).into()),
        }
    }

    pub async fn upsert(clients: Vec<Self>, db: &SurrealDB) -> Result<Vec<Self>, ApiError> {
        Ok(DBClient::bulk(clients, db)
            .await?
            .iter()
            .map(Self::from)
            .collect())
    }
}

impl From<&DBClient> for Client {
    fn from(db: &DBClient) -> Self {
        Self {
            id: DBClient::record_id_to_uuid(&db.id),
            login: db.login.clone(),
            age: db.age,
            location: db.location.clone(),
            gender: db.gender.clone(),
        }
    }
}
