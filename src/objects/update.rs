use super::{CallBackQuery, Message};

/// Represents an update returned by the Telegram API.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub inline_query: Option<String>,
    pub chosen_inline_result: Option<String>,
    pub callback_query: Option<CallBackQuery>,
}
