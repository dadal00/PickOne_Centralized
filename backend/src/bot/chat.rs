use super::{
    models::{ChatMessage, Command, HandlerResult, RedisBotAction},
    photo::{download_photo, process_photos},
    redis::get_num_photos,
};
use crate::{api::redis::remove_id, config::read_secret, error::AppError, state::AppState};
use dptree::case;
use std::{error::Error as stdErr, sync::Arc};
use teloxide::{
    dispatching::UpdateHandler, filter_command, prelude::*, types::FileMeta,
    utils::command::BotCommands,
};
use tracing::info;

pub async fn start_bot(state: Arc<AppState>) -> Result<(), AppError> {
    let bot = Bot::new(
        read_secret("RUST_BOT_CUTS_KEY")
            .inspect_err(|_| {
                info!("RUST_BOT_CUTS_KEY not set, using default");
            })
            .unwrap_or_else(|_| "its so over".into()),
    );

    tokio::spawn(async move {
        Dispatcher::builder(bot, schema())
            .dependencies(dptree::deps![state.clone()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    });

    Ok(())
}

fn schema() -> UpdateHandler<Box<dyn stdErr + Send + Sync + 'static>> {
    let command_handler = filter_command::<Command, _>()
        .branch(case![Command::Process].endpoint(process))
        .branch(case![Command::Clear].endpoint(clear));

    Update::filter_message()
        .branch(command_handler)
        .endpoint(listen)
}

async fn clear(bot: Bot, msg: Message, state: Arc<AppState>) -> HandlerResult {
    bot.send_message(msg.chat.id, ChatMessage::Cleared.as_ref())
        .await?;

    remove_id(
        state.clone(),
        &format!(
            "{}:{}",
            RedisBotAction::User.as_ref(),
            &msg.from.unwrap().id.to_string()
        ),
    )
    .await?;

    Ok(())
}

async fn process(bot: Bot, msg: Message, state: Arc<AppState>) -> HandlerResult {
    let num_photos = get_num_photos(
        state.clone(),
        RedisBotAction::User.as_ref(),
        &msg.from.clone().unwrap().id.to_string(),
    )
    .await?;

    if num_photos < state.config.bot_num_pictures {
        bot.send_message(
            msg.chat.id,
            format!("{}{}", num_photos, ChatMessage::Received.as_ref()),
        )
        .await?;

        return Ok(());
    }

    bot.send_message(msg.chat.id, ChatMessage::Processing.as_ref())
        .await?;

    bot.send_message(
        msg.chat.id,
        format!(
            "{} {}/{}",
            ChatMessage::Processed.as_ref(),
            state.config.bot_photo_url,
            process_photos(
                state.clone(),
                RedisBotAction::User,
                &msg.from.unwrap().id.to_string()
            )
            .await?
        ),
    )
    .await?;

    Ok(())
}

async fn listen(bot: Bot, msg: Message, state: Arc<AppState>) -> HandlerResult {
    let photos = msg.photo();

    if photos.is_none() {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await?;

        return Ok(());
    }

    let file: &FileMeta = &photos.expect("is_none failed").last().unwrap().file;

    if file.size > state.config.bot_max_bytes {
        bot.send_message(msg.chat.id, ChatMessage::ImageTooLarge.as_ref())
            .await?;

        return Ok(());
    }

    let msg_clone = msg.clone();

    let length = download_photo(
        &bot,
        state.clone(),
        RedisBotAction::User,
        &msg_clone.from.unwrap().id.to_string(),
        file.id.clone(),
    )
    .await?;

    bot.send_message(
        msg.chat.id,
        format!("{}{}", length, ChatMessage::Received.as_ref()),
    )
    .await?;

    Ok(())
}
