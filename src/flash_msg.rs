use rocket::request::FlashMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashMsg {
    pub kind: String,
    pub message: String,
}

impl From<FlashMessage<'_>> for FlashMsg {
    fn from(flash: rocket::response::Flash<&rocket::http::CookieJar<'_>>) -> Self {
        FlashMsg {
            kind: flash.kind().to_string(),
            message: flash.message().to_string(),
        }
    }
}
