use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum UpdateType<'a> {
    Message(&'a Message),
    EditedMessage(&'a Message),
    CallbackQuery(&'a CallbackQuery),
    Unknown,
}

#[derive(Serialize, Debug)]
pub struct SendMessageRequest<'a> {
    pub chat_id: i64,
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TelegramResponse<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Update {
    pub update_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
}

impl Update {
    pub fn get_type(&self) -> UpdateType {
        match (&self.message, &self.edited_message, &self.callback_query) {
            (Some(msg), None, None) => UpdateType::Message(msg),
            (None, Some(msg), None) => UpdateType::EditedMessage(msg),
            (None, None, Some(callback)) => UpdateType::CallbackQuery(callback),
            _ => UpdateType::Unknown,
        } 
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub message_id: i64,
    pub date: i64,
    pub chat: Chat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: i32,
}
