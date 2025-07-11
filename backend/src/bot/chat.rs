use super::{
    models::{ChatDialogue, ChatMessage, Command, HandlerResult},
    state::BotState,
};
use crate::{AppError, config::read_secret};
use dptree::case;
use std::{collections::VecDeque, error::Error as stdErr};
use teloxide::{
    dispatching::{UpdateHandler, dialogue, dialogue::InMemStorage},
    filter_command,
    prelude::*,
    utils::command::BotCommands,
};
use tracing::info;

pub async fn start_bot() -> Result<(), AppError> {
    let bot = Bot::new(
        read_secret("RUST_BOT_CUTS_KEY")
            .inspect_err(|_| {
                info!("RUST_BOT_CUTS_KEY not set, using default");
            })
            .unwrap_or_else(|_| "its so over".into()),
    );

    tokio::spawn(async move {
        Dispatcher::builder(bot, schema())
            .dependencies(dptree::deps![InMemStorage::<BotState>::new()])
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

    let message_handler = Update::filter_message().branch(
        case![BotState::ReceivingPhotos { file_ids }]
            .branch(command_handler)
            .endpoint(listen),
    );

    dialogue::enter::<Update, InMemStorage<BotState>, BotState, _>().branch(message_handler)
}

async fn clear(
    bot: Bot,
    dialogue: ChatDialogue,
    mut file_ids: VecDeque<String>,
    msg: Message,
) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        format!("{}{}", file_ids.len(), ChatMessage::Cleared.as_ref()),
    )
    .await?;

    file_ids.clear();

    dialogue
        .update(BotState::ReceivingPhotos { file_ids })
        .await?;

    Ok(())
}

async fn process(
    bot: Bot,
    dialogue: ChatDialogue,
    file_ids: VecDeque<String>,
    msg: Message,
) -> HandlerResult {
    if file_ids.len() < 4 {
        bot.send_message(
            msg.chat.id,
            format!("{}{}", file_ids.len(), ChatMessage::Received.as_ref()),
        )
        .await?;

        dialogue
            .update(BotState::ReceivingPhotos { file_ids })
            .await?;

        return Ok(());
    }

    bot.send_message(msg.chat.id, ChatMessage::Processing.as_ref())
        .await?;

    dialogue
        .update(BotState::ReceivingPhotos { file_ids })
        .await?;

    Ok(())
}

async fn listen(
    bot: Bot,
    dialogue: ChatDialogue,
    mut file_ids: VecDeque<String>,
    msg: Message,
) -> HandlerResult {
    let photos = msg.photo();

    if photos.is_some() {
        if file_ids.len() >= 4 {
            file_ids.pop_front();
        }

        file_ids.push_back(
            photos
                .expect("is_some failed")
                .last()
                .unwrap()
                .file
                .id
                .to_string(),
        );

        bot.send_message(
            msg.chat.id,
            format!("{}{}", file_ids.len(), ChatMessage::Received.as_ref()),
        )
        .await?;

        dialogue
            .update(BotState::ReceivingPhotos { file_ids })
            .await?;

        return Ok(());
    }

    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;

    dialogue
        .update(BotState::ReceivingPhotos { file_ids })
        .await?;

    Ok(())
}
