use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EditAboutUs<'a> {
    pub about_us: &'a str,
}
