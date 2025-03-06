use utoipa_actix_web::{scope, service_config::ServiceConfig};

mod by_id;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/campaigns").configure(by_id::config));
}
