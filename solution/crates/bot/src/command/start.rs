use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    Bot,
};

use crate::state::{BotDialogue, HandlerResult};

pub async fn start(bot: Bot, dialogue: BotDialogue) -> HandlerResult {
    let buttons = [("Войти", "login"), ("Зарегистрироваться", "register")].map(|(label, id)| {
        [InlineKeyboardButton::callback(
            label.to_owned(),
            id.to_owned(),
        )]
    });

    bot.send_message(dialogue.chat_id(), "Start")
        .reply_markup(InlineKeyboardMarkup::new(buttons))
        .await?;
    Ok(())
}
