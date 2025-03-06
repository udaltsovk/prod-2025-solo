use utoipa_actix_web::{scope, service_config::ServiceConfig};

mod bulk;
mod by_id;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/clients")
            .service(by_id::get_handler)
            .service(bulk::post_handler),
    );
}
