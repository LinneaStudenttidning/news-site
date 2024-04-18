use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCreator<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
    pub password: &'a str,
    pub as_publisher: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PromoteOrDemote<'a> {
    pub username: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProfile<'a> {
    pub display_name: Option<&'a str>,
    pub biography: Option<&'a str>,
}
