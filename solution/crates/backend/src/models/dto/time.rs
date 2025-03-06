use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use surrealdb_helper::SurrealDB;
use utoipa::ToSchema;
use validator::Validate;

use crate::{models::db::DBTime, routes::ApiError};

static CURRENT_DAY: RwLock<i32> = RwLock::new(1);

#[derive(Deserialize, Serialize, ToSchema, Validate, Clone, Debug)]
pub struct Time {
    #[schema(default = 1, minimum = 1)]
    #[validate(range(min = 1))]
    /// Текущий день (целое число).
    pub current_date: i32,
}

impl Time {
    pub fn get() -> Self {
        Self {
            current_date: CURRENT_DAY.read().unwrap().clone(),
        }
    }

    pub async fn advance_day(self, db: &SurrealDB) -> Result<Self, ApiError> {
        match (Self::get().current_date, self.current_date) {
            (cd, nd) if cd > nd => {
                return Err(ApiError::InvalidInput(
                    format!("New date can't be earlier than current date {cd:?} {nd:?}")
                        .to_string(),
                ))
            }
            (cd, nd) if cd < nd => {
                let mut current_day = CURRENT_DAY.write().unwrap();
                *current_day = DBTime::upsert(self, db).await?.current_date;
            }
            _ => (),
        };

        Ok(Self::get())
    }

    pub fn cleanup() {
        *CURRENT_DAY.write().unwrap() = 1;
    }
}

impl From<DBTime> for Time {
    fn from(db: DBTime) -> Self {
        Self {
            current_date: db.current_date,
        }
    }
}
