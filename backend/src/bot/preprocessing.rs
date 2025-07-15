use super::{
    models::{Icon, Icons, PhotoBox},
    photo::overlay_photo,
};
use axum::http::{HeaderValue, header::HeaderMap};
use image::{DynamicImage, GenericImageView, RgbaImage};
use once_cell::sync::Lazy;
use std::env;
use tracing::warn;

pub static BACKGROUND_FOLDER: Lazy<String> = Lazy::new(|| {
    env::var("RUST_BOT_BACKGROUND_CONTAINER_FOLDER_PATH").unwrap_or_else(|_| {
        warn!("Env var RUST_BOT_BACKGROUND_CONTAINER_FOLDER_PATH not found, using default");
        "/assets".to_string()
    })
});

pub static PHOTO_BACKGROUND: Lazy<DynamicImage> = Lazy::new(|| {
    image::open(format!(
        "{}{}",
        *BACKGROUND_FOLDER,
        env::var("RUST_BOT_BACKGROUND_FILE").unwrap_or_else(|_| {
            warn!("Env var RUST_BOT_BACKGROUND_FILE not found, using default");
            "/photo_strip_background.jpg".to_string()
        })
    ))
    .expect("Image failed to load")
});

pub static PHOTO_BACKGROUND_SIZE: Lazy<(u32, u32)> = Lazy::new(|| PHOTO_BACKGROUND.dimensions());

pub static PHOTO_ICONS: Lazy<DynamicImage> = Lazy::new(|| {
    let (width, height) = *PHOTO_BACKGROUND_SIZE;
    let mut layer = DynamicImage::ImageRgba8(RgbaImage::new(width, height));

    let pete = image::open(format!(
        "{}{}",
        *BACKGROUND_FOLDER,
        env::var("RUST_BOT_PETE_FILE").unwrap_or_else(|_| {
            warn!("Env var RUST_BOT_PETE_FILE not found, using default");
            "/pete.png".to_string()
        })
    ))
    .expect("Image failed to load");

    let tower = image::open(format!(
        "{}{}",
        *BACKGROUND_FOLDER,
        env::var("RUST_BOT_TOWER_FILE").unwrap_or_else(|_| {
            warn!("Env var RUST_BOT_TOWER_FILE not found, using default");
            "/tower.png".to_string()
        })
    ))
    .expect("Image failed to load");

    let fountain = image::open(format!(
        "{}{}",
        *BACKGROUND_FOLDER,
        env::var("RUST_BOT_FOUNTAIN_FILE").unwrap_or_else(|_| {
            warn!("Env var RUST_BOT_FOUNTAIN_FILE not found, using default");
            "/fountain.png".to_string()
        })
    ))
    .expect("Image failed to load");

    overlay_photo(&mut layer, &pete, ICONS.pete.center.0, ICONS.pete.center.1);
    overlay_photo(
        &mut layer,
        &tower,
        ICONS.tower.center.0,
        ICONS.tower.center.1,
    );
    overlay_photo(
        &mut layer,
        &fountain,
        ICONS.fountain.center.0,
        ICONS.fountain.center.1,
    );

    layer
});

pub static JPEG_HEADER: Lazy<HeaderMap> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("image/jpeg"));
    headers.insert(
        "Content-Disposition",
        HeaderValue::from_str("attachment; filename=\"photo_strip.jpg\"").unwrap(),
    );
    headers
});

pub const PHOTO_BOXES: [PhotoBox; 4] = [
    PhotoBox {
        center: (312, 329),
        max_width: 449,
        max_height: 451,
    },
    PhotoBox {
        center: (767, 329),
        max_width: 449,
        max_height: 451,
    },
    PhotoBox {
        center: (312, 785),
        max_width: 449,
        max_height: 451,
    },
    PhotoBox {
        center: (767, 785),
        max_width: 449,
        max_height: 451,
    },
];

pub const ICONS: Icons = Icons {
    pete: Icon { center: (116, 916) },
    tower: Icon { center: (948, 216) },
    fountain: Icon { center: (974, 910) },
};
