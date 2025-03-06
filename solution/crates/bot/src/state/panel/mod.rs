use reqwest::{Client, StatusCode};
use serde_json::json;
use teloxide::{
    dispatching::UpdateFilterExt,
    dptree::{self, case},
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{CallbackQuery, ChatId, InlineKeyboardButton, InlineKeyboardMarkup, Message, Update},
    Bot,
};
use uuid::Uuid;

use crate::config;

use super::{BotDialogue, Handler, HandlerResult, State};

#[derive(Clone)]
pub enum PrePanelState {
    PanelMenu {
        advertiser_id: Uuid,
        advertiser_name: String,
    },
    Register,
    Login,
}

pub struct PanelState;

impl PanelState {
    pub fn handler() -> Handler {
        dptree::entry()
            .branch(
                Update::filter_callback_query().branch(
                    dptree::filter(|q: CallbackQuery| q.data.unwrap_or_default() == "exit")
                        .endpoint(exit),
                ),
            )
            .branch(
                Update::filter_message().branch(case![State::Panel { previous }].endpoint(enter)),
            )
    }
}

async fn enter(
    bot: Bot,
    dialogue: BotDialogue,
    previous: PrePanelState,
    msg: Message,
) -> HandlerResult {
    match previous {
        PrePanelState::PanelMenu {
            advertiser_id,
            advertiser_name,
        } => {
            welcome_message(
                &bot,
                dialogue.chat_id(),
                &advertiser_id.to_string(),
                &advertiser_name,
            )
            .await?;
        }
        PrePanelState::Register => match msg.text() {
            Some(text) => {
                match Client::new()
                    .post(format!(
                        "http://{}/advertisers/bulk",
                        config::BACKEND_ADDRESS.as_str()
                    ))
                    .json(&json!([
                            {
                                "advertiser_id": Uuid::now_v7(),
                                "name": text
                            }
                    ]))
                    .send()
                    .await
                {
                    Ok(resp) => match resp.status() {
                        StatusCode::CREATED => {
                            let uuid = resp.json::<serde_json::Value>().await.unwrap();
                            let uuid = uuid.as_array().unwrap()[0]
                                .get("advertiser_id")
                                .unwrap()
                                .as_str()
                                .unwrap();

                            welcome_message(&bot, dialogue.chat_id(), uuid, text).await?;
                        }
                        _ => {
                            bot.send_message(
                                dialogue.chat_id(),
                                "Не удалось найти рекламодателя с указанным UUID. ",
                            )
                            .await?;
                        }
                    },
                    Err(..) => {
                        bot.send_message(dialogue.chat_id(), "Не удалось создать рекламодателя.")
                            .await?;
                    }
                }
            }
            None => {
                bot.send_message(
                    dialogue.chat_id(),
                    "Пожалуйста, введите название рекламодателя.",
                )
                .await?;
                return Ok(());
            }
        },
        PrePanelState::Login => match msg.text() {
            Some(text) => {
                match Client::new()
                    .get(format!(
                        "http://{}/advertisers/{text}",
                        config::BACKEND_ADDRESS.as_str()
                    ))
                    .send()
                    .await
                {
                    Ok(resp) => match resp.status() {
                        StatusCode::OK => {
                            let name = resp.json::<serde_json::Value>().await.unwrap();
                            let name = name.get("name").unwrap().as_str().unwrap();

                            welcome_message(&bot, dialogue.chat_id(), text, name).await?;
                        }
                        _ => {
                            bot.send_message(
                                dialogue.chat_id(),
                                "Не удалось найти рекламодателя с указанным UUID. ",
                            )
                            .await?;
                        }
                    },
                    Err(..) => {
                        bot.send_message(
                            dialogue.chat_id(),
                            "Не удалось найти рекламодателя с указанным UUID. ",
                        )
                        .await?;
                    }
                }
            }
            None => {
                bot.send_message(
                    dialogue.chat_id(),
                    "Пожалуйста, введите UUID рекламодателя.",
                )
                .await?;
                return Ok(());
            }
        },
    }

    Ok(())
}

async fn exit(_bot: Bot, dialogue: BotDialogue) -> HandlerResult {
    dialogue.update(State::Start).await?;
    Ok(())
}

async fn welcome_message(
    bot: &Bot,
    chat_id: ChatId,
    advertiser_id: &str,
    advertiser_name: &str,
) -> Result<Message, teloxide::RequestError> {
    bot.send_message(
        chat_id,
        format!("Доброго времени суток, \"{advertiser_name}\"! (UUID: {advertiser_id})"),
    )
    .reply_markup(InlineKeyboardMarkup::new([
        [InlineKeyboardButton::callback(
            "Аккаунт",
            format!("account::{advertiser_id}::"),
        )],
        [InlineKeyboardButton::callback(
            "Мои кампании",
            format!("campaigns::{advertiser_id}::"),
        )],
        [InlineKeyboardButton::callback(
            "Статистика",
            format!("campaigns::{advertiser_id}::"),
        )],
        [InlineKeyboardButton::callback("Выйти из аккаунта", "exit")],
    ]))
    .await
}
