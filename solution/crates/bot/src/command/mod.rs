use super::state::State;
use crate::state::Handler;
use teloxide::{
    dispatching::UpdateFilterExt, dptree, filter_command, macros::BotCommands,
    prelude::dptree::case, types::Update,
};

mod start;

use start::start;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    Start,
}

pub struct Commands;

impl Commands {
    pub fn handler() -> Handler {
        dptree::entry().branch(
            Update::filter_message().branch(
                filter_command::<Command, _>()
                    .branch(case![State::Start].branch(case![Command::Start].endpoint(start))),
            ),
        )
    }
}
