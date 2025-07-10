use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(
        description = "format the 4 most recent pictures sent in the last 24hrs into a photo strip."
    )]
    Process,
}
