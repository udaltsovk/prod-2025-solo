use utoipa_actix_web::{scope, service_config::ServiceConfig};

pub mod click;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/{ad_id}").service(click::post_handler));
}
