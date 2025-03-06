use crate::command::Commands;
use login::LoginState;
use panel::{PanelState, PrePanelState};
use register::RegisterState;
use std::error::Error;
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        DpHandlerDescription, UpdateHandler,
    },
    dptree::{endpoint, Handler as DPTreeHandler},
    prelude::{DependencyMap, Dialogue, Requester},
    types::{Message, Update},
    Bot,
};
use uuid::Uuid;

pub mod login;
mod panel;
pub mod register;

pub type Handler = DPTreeHandler<'static, DependencyMap, HandlerResult, DpHandlerDescription>;
pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
pub type BotDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,

    // Создание нового рекламодателя
    Register,
    ReceiveAdvertiserName,

    // Вход в панель
    Login,
    ReceiveAdvertiserUUID,

    // Панель
    Panel {
        previous: PrePanelState,
    },

    // Меню аккаунта
    Account {
        advertiser_id: Uuid,
        advertiser_name: String,
    },

    // Меню кампаний
    Campaigns {
        advertiser_id: Uuid,
        advertiser_name: String,
    },

    // Меню статистики
    Stats {
        advertiser_id: Uuid,
        advertiser_name: String,
    },
}

pub fn schema() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(Commands::handler())
        .branch(RegisterState::handler())
        .branch(LoginState::handler())
        .branch(PanelState::handler())
        .branch(endpoint(invalid_state))
}

pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Не удалось обработать сообщение. Напишите /start для возврата в диалог.",
    )
    .await?;
    Ok(())
}
