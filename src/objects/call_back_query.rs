use super::Message;
use super::User;

/// Represents an incoming callback query from a `InlineKeyboardButton` in an `InlineKeyboardMarkup`.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CallBackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}
