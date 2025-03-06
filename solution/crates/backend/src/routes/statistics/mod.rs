use utoipa_actix_web::{scope, service_config::ServiceConfig};

mod advertisers;
mod campaigns;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/statistics")
            .configure(campaigns::config)
            .configure(advertisers::config),
    );
}
