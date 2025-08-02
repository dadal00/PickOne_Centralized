use super::{
    models::{ChatMessage, Command, HandlerResult},
    photo::generate_qr_bytes,
    utilities::{download_user_photo, user_clear, user_get_link, user_num_photos},
};
use crate::{AppError, AppState, config::read_secret};
use dptree::case;
use std::{error::Error as stdErr, sync::Arc};
use teloxide::{
    dispatching::UpdateHandler,
    filter_command,
    prelude::*,
    types::{FileMeta, InputFile},
    utils::command::BotCommands,
};

pub async fn start_bot(state: Arc<AppState>) -> Result<(), AppError> {
    let bot = Bot::new(read_secret("RUST_BOT_CUTS_KEY").unwrap_or_else(|e| {
        panic!("Failed to load RUST_BOT_CUTS_KEY: {e}");
    }));

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

    user_clear(state, &msg).await?;

    Ok(())
}

async fn process(bot: Bot, msg: Message, state: Arc<AppState>) -> HandlerResult {
    let num_photos = user_num_photos(state.clone(), &msg).await?;

    if num_photos < state.config.bot.num_pictures {
        bot.send_message(
            msg.chat.id,
            format!("{}{}", num_photos, ChatMessage::Received.as_ref()),
        )
        .await?;

        return Ok(());
    }

    bot.send_message(msg.chat.id, ChatMessage::Processing.as_ref())
        .await?;

    let link = user_get_link(state, &msg).await?;

    bot.send_photo(msg.chat.id, InputFile::memory(generate_qr_bytes(&link)?))
        .caption(format!("{}\n\n{}", ChatMessage::Processed.as_ref(), &link))
        .send()
        .await?;

    Ok(())
}

async fn listen(bot: Bot, msg: Message, state: Arc<AppState>) -> HandlerResult {
    if msg.photo().is_none() {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await?;

        return Ok(());
    }

    let file: &FileMeta = &msg.photo().expect("is_none failed").last().unwrap().file;

    if file.size > state.config.bot.max_bytes {
        bot.send_message(msg.chat.id, ChatMessage::ImageTooLarge.as_ref())
            .await?;

        return Ok(());
    }

    bot.send_message(
        msg.chat.id,
        format!(
            "{}{}",
            download_user_photo(&bot, state, &msg, file).await?,
            ChatMessage::Received.as_ref()
        ),
    )
    .await?;

    Ok(())
}
