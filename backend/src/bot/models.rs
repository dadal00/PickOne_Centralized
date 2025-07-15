use std::error::Error as stdErr;
use strum_macros::{AsRefStr, EnumString};
use teloxide::utils::command::BotCommands;

pub type HandlerResult = Result<(), Box<dyn stdErr + Send + Sync>>;

#[derive(BotCommands, Clone, PartialEq)]
#[command(
    rename_rule = "lowercase",
    description = "These are the commands are supported:"
)]
pub enum Command {
    #[command(
        description = "Create a photo strip with your 4 most recent pictures sent to me in the last 24 hours."
    )]
    Process,

    #[command(description = "Clear pictures.")]
    Clear,
}

#[derive(EnumString, AsRefStr)]
pub enum ChatMessage {
    #[strum(serialize = "/4 image(s) sent")]
    Received,

    #[strum(serialize = "All images cleared")]
    Cleared,

    #[strum(serialize = "Processing...")]
    Processing,

    #[strum(serialize = "Image too large")]
    ImageTooLarge,

    #[strum(serialize = "Heres your photo:")]
    Processed,
}

#[derive(EnumString, AsRefStr, PartialEq)]
pub enum RedisBotAction {
    #[strum(serialize = "user_id")]
    User,

    #[strum(serialize = "qr")]
    QRPicture,
}
