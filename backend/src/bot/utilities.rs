use super::{
    models::RedisBotAction,
    photo::{download_photo, process_photos},
    redis::get_num_photos,
};
use crate::{AppError, AppState, microservices::redis::remove_id};
use anyhow::Result as anyResult;
use std::sync::Arc;
use teloxide::{prelude::*, types::FileMeta};

pub async fn user_clear(state: Arc<AppState>, msg: &Message) -> Result<(), AppError> {
    remove_id(
        state,
        &format!(
            "{}:{}",
            RedisBotAction::User.as_ref(),
            msg.from.as_ref().unwrap().id
        ),
    )
    .await?;

    Ok(())
}

pub async fn user_num_photos(state: Arc<AppState>, msg: &Message) -> Result<u8, AppError> {
    get_num_photos(
        state,
        RedisBotAction::User.as_ref(),
        &msg.from.as_ref().unwrap().id.to_string(),
    )
    .await
}

pub async fn user_get_link(state: Arc<AppState>, msg: &Message) -> Result<String, AppError> {
    Ok(format!(
        "{}/{}",
        state.config.bot.photo_url,
        process_photos(
            state.clone(),
            RedisBotAction::User,
            &msg.from.as_ref().unwrap().id.to_string()
        )
        .await?
    ))
}

pub async fn download_user_photo(
    bot: &Bot,
    state: Arc<AppState>,
    msg: &Message,
    file: &FileMeta,
) -> anyResult<u8> {
    download_photo(
        bot,
        state,
        RedisBotAction::User,
        &msg.from.as_ref().unwrap().id.to_string(),
        file.id.clone(),
    )
    .await
}
