use surrealdb::RecordId;
use uuid::Uuid;

mod advertiser;
mod campaign;
mod client;
mod interaction;
mod ml_score;
pub mod stats;
mod time;

pub use advertiser::DBAdvertiser;
pub use campaign::{DBCampaign, DBTargeting};
pub use client::DBClient;
pub use interaction::DBInteraction;
pub use ml_score::DBMLScore;
pub use time::DBTime;

pub trait DBModel {
    const TABLE: &'static str;

    fn record_id_to_uuid(record_id: &RecordId) -> Uuid {
        let wrapped_id = &record_id.key().to_string();
        let original_id: &str = wrapped_id
            .strip_prefix('⟨')
            .unwrap()
            .strip_suffix('⟩')
            .unwrap();
        Uuid::parse_str(original_id).unwrap()
    }

    fn record_id_from_uuid(uuid: &Uuid) -> RecordId {
        RecordId::from_table_key(Self::TABLE, uuid.to_string())
    }
}

pub trait DBRelation: DBModel {
    fn relation_id(r#in: Uuid, out: Uuid) -> RecordId {
        RecordId::from_table_key(Self::TABLE, format!("{}_{}", r#in, out))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use surrealdb::RecordId;

    use super::DBModel;

    struct MockDBModel;

    impl DBModel for MockDBModel {
        const TABLE: &str = "mock_model";
    }

    #[rstest]
    #[case::normal_uuid("3fa85f64-5717-4562-b3fc-2c963f66afa6")]
    #[should_panic]
    #[case::not_uuid("idk")]
    fn record_id_to_uuid(#[case] original_uuid: &str) {
        let record_id = RecordId::from_table_key(MockDBModel::TABLE, original_uuid);

        let uuid = MockDBModel::record_id_to_uuid(&record_id);

        assert_eq!(uuid.to_string(), original_uuid);
    }
}
