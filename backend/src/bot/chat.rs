use super::models::Command;
use crate::{AppError, config::read_secret};
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::info;

pub async fn run_bot() -> Result<(), AppError> {
    let bot = Bot::new(
        read_secret("RUST_BOT_CUTS_KEY")
            .inspect_err(|_| {
                info!("RUST_BOT_CUTS_KEY not set, using default");
            })
            .unwrap_or_else(|_| "its so over".into()),
    );
    tokio::spawn(async move {
        Command::repl(bot, answer).await;
    });

    Ok(())
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Process => {
            bot.send_message(msg.chat.id, "WIP. Will process eventually.")
                .await?
        }
    };

    Ok(())
}
