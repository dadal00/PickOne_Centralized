use super::{
    models::RedisBotAction,
    preprocessing::{
        JPEG_HEADER, PHOTO_BACKGROUND, PHOTO_BACKGROUND_SIZE, PHOTO_BOXES, PHOTO_ICONS,
    },
    redis::{get_bytes, get_vector_of_bytes, insert_formatted_photo, insert_user_photo_bytes},
};
use crate::{AppError, AppState, error::BotError};
use anyhow::Result as anyResult;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::TryStreamExt;
use image::{
    DynamicImage, GenericImageView, ImageFormat,
    ImageFormat::{Jpeg, Png},
    Luma, imageops,
    imageops::FilterType::Lanczos3,
    load_from_memory,
};
use qrcode::QrCode;
use std::{io::Cursor, sync::Arc};
use teloxide::{Bot, net::Download, prelude::*, types::FileId};
use uuid::Uuid;

pub async fn process_photos(
    state: Arc<AppState>,
    redis_bot_action: RedisBotAction,
    user_id: &str,
) -> Result<String, AppError> {
    let photos: Vec<DynamicImage> =
        get_all_photos(state.clone(), redis_bot_action.as_ref(), user_id).await?;

    let mut result = PHOTO_BACKGROUND.clone();

    for (photo, box_) in photos.iter().zip(PHOTO_BOXES.iter()) {
        overlay_photo(
            &mut result,
            &resize_photo(photo, box_.max_width, box_.max_height),
            box_.center.0,
            box_.center.1,
        );
    }

    let (width, height) = *PHOTO_BACKGROUND_SIZE;
    overlay_photo(&mut result, &PHOTO_ICONS, width / 2, height / 2);

    let qr_id = Uuid::new_v4().to_string();

    insert_formatted_photo(
        state.clone(),
        RedisBotAction::QRPicture.as_ref(),
        &qr_id,
        &photo_to_bytes(&result, Jpeg)?,
        RedisBotAction::User.as_ref(),
        user_id,
    )
    .await?;

    Ok(qr_id)
}

fn resize_photo(image: &DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
    let (width, height) = image.dimensions();

    let scale = f32::min(
        max_width as f32 / width as f32,
        max_height as f32 / height as f32,
    );

    image.resize(
        (width as f32 * scale) as u32,
        (height as f32 * scale) as u32,
        Lanczos3,
    )
}

pub fn overlay_photo(
    base: &mut DynamicImage,
    overlay: &DynamicImage,
    center_x: u32,
    center_y: u32,
) {
    let (overlay_width, overlay_height) = overlay.dimensions();

    imageops::overlay(
        base,
        overlay,
        center_x.saturating_sub(overlay_width / 2).into(),
        center_y.saturating_sub(overlay_height / 2).into(),
    );
}

async fn get_all_photos(
    state: Arc<AppState>,
    key_prefix: &str,
    user_id: &str,
) -> anyResult<Vec<DynamicImage>> {
    let photos_bytes: Vec<Vec<u8>> =
        get_vector_of_bytes(state.clone(), key_prefix, user_id).await?;

    let mut images = Vec::with_capacity(photos_bytes.len());

    for photo_bytes in photos_bytes {
        images.push(load_from_memory(&photo_bytes)?);
    }

    Ok(images)
}

pub async fn download_photo(
    bot: &Bot,
    state: Arc<AppState>,
    redis_bot_action: RedisBotAction,
    user_id: &str,
    file_id: FileId,
) -> anyResult<u8> {
    Ok(insert_user_photo_bytes(
        state.clone(),
        redis_bot_action.as_ref(),
        user_id,
        &bot.download_file_stream(&bot.get_file(file_id).await?.path)
            .try_fold(Vec::new(), |mut acc, chunk| async move {
                acc.extend_from_slice(&chunk);
                Ok(acc)
            })
            .await?,
    )
    .await?)
}

pub fn photo_to_bytes(image: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>, BotError> {
    let mut bytes: Vec<u8> = Vec::new();

    image.write_to(&mut Cursor::new(&mut bytes), format)?;

    Ok(bytes)
}

pub async fn photo_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match get_bytes(state.clone(), RedisBotAction::QRPicture.as_ref(), &id).await? {
        Some(bytes) => Ok((
            StatusCode::OK,
            JPEG_HEADER.clone(),
            photo_to_bytes(
                &load_from_memory(&bytes).map_err(|e| AppError::Bot(e.into()))?,
                Jpeg,
            )?,
        )
            .into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Not found").into_response()),
    }
}

pub fn generate_qr_bytes(link: &str) -> Result<Vec<u8>, BotError> {
    photo_to_bytes(
        &DynamicImage::ImageLuma8(
            QrCode::new(link)
                .expect("QR generation failed")
                .render::<Luma<u8>>()
                .max_dimensions(512, 512)
                .build(),
        ),
        Png,
    )
}
