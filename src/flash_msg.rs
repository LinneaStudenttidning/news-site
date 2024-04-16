use rocket::request::FlashMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashMsg {
    pub kind: String,
    pub message: String,
}

impl From<Option<FlashMessage<'_>>> for FlashMsg {
    fn from(
        flash: std::option::Option<rocket::response::Flash<&rocket::http::CookieJar<'_>>>,
    ) -> Self {
        match flash {
            None => FlashMsg {
                kind: "".to_string(),
                message: "".to_string(),
            },
            Some(msg) => FlashMsg {
                kind: msg.kind().to_string(),
                message: msg.message().to_string(),
            },
        }
    }
}
