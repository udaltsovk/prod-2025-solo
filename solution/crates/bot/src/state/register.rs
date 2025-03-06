use super::{panel::PrePanelState, BotDialogue, Handler, HandlerResult};
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    prelude::Requester,
    types::{CallbackQuery, Update},
    Bot,
};

pub struct RegisterState;
impl RegisterState {
    pub fn handler() -> Handler {
        dptree::entry().branch(
            Update::filter_callback_query().branch(
                dptree::filter(|q: CallbackQuery| q.data.unwrap_or_default() == "register")
                    .endpoint(enter),
            ),
        )
    }
}

pub async fn enter(bot: Bot, dialogue: BotDialogue, q: CallbackQuery) -> HandlerResult {
    if let Some(_) = &q.data {
        bot.send_message(dialogue.chat_id(), "Введите название рекламодателя:")
            .await?;
        dialogue
            .update(super::State::Panel {
                previous: PrePanelState::Register,
            })
            .await?;
    }

    Ok(())
}
