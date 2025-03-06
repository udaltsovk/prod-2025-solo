use utoipa_actix_web::{scope, service_config::ServiceConfig};

mod advance;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/time").service(advance::post_handler));
}
