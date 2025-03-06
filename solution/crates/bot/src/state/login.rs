use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    prelude::Requester,
    types::{CallbackQuery, Update},
    Bot,
};

use super::{panel::PrePanelState, BotDialogue, Handler, HandlerResult};

pub struct LoginState;
impl LoginState {
    pub fn handler() -> Handler {
        dptree::entry().branch(
            Update::filter_callback_query().branch(
                dptree::filter(|q: CallbackQuery| q.data.unwrap_or_default() == "login")
                    .endpoint(enter),
            ),
        )
    }
}

async fn enter(bot: Bot, dialogue: BotDialogue, q: CallbackQuery) -> HandlerResult {
    if let Some(_) = &q.data {
        bot.send_message(dialogue.chat_id(), "Введите UUID рекламодателя:")
            .await?;
        dialogue
            .update(super::State::Panel {
                previous: PrePanelState::Login,
            })
            .await?;
    }

    Ok(())
}
