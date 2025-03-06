use env_config::environment_variables;
use state::State;
use teloxide::{dispatching::dialogue::InMemStorage, dptree, prelude::DependencyMap};

pub mod command;
pub mod state;

environment_variables! {
    BACKEND_ADDRESS: String = "localhost:8080",
}

pub fn bot_setup() -> DependencyMap {
    config::init();
    log::info!("Starting ad_platform bot...");
    dptree::deps![InMemStorage::<State>::new()]
}
