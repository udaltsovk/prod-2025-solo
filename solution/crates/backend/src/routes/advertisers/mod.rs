use utoipa_actix_web::{scope, service_config::ServiceConfig};

mod bulk;
mod by_id;
mod campaigns;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/advertisers")
            .service(by_id::get_handler)
            .service(bulk::post_handler)
            .configure(campaigns::config),
    );
}
