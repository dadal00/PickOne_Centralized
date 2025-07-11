use std::collections::VecDeque;

#[derive(Clone)]
pub enum BotState {
    ReceivingPhotos { file_ids: VecDeque<String> },
}

impl Default for BotState {
    fn default() -> Self {
        BotState::ReceivingPhotos {
            file_ids: VecDeque::new(),
        }
    }
}
