use super::User;

/// Represents a Telegram message entity.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    type_message_entity: String,
    offset: i64,
    length: i64,
    url: Option<String>,
    user: Option<User>,
}
