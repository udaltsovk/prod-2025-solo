use bot::{bot_setup, state::schema};
use env_logger::Env;
use teloxide::{prelude::Dispatcher, Bot};

#[tokio::main]
async fn main() {
    let default_log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    env_logger::init_from_env(Env::default().default_filter_or(default_log_level));

    let bot_deps = bot_setup();

    Dispatcher::builder(Bot::from_env(), schema())
        .dependencies(bot_deps)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
