use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordSelf<'a> {
    pub current_password: &'a str,
    pub new_password: &'a str,
    pub repeat_new_password: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordOther<'a> {
    pub username: &'a str,
    pub new_password: &'a str,
}
