use crate::{AppError, config::try_load};

pub async fn init_housing() -> Result<(), AppError> {
    let housing_file = format!(
        "{}{}",
        try_load::<String>("RUST_CONTAINER_FOLDER_PATH", "/assets").unwrap(),
        try_load::<String>("RUST_HOUSING_FILE", "/housing.json").unwrap()
    );

    Ok(())
}
