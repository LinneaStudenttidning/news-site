use serde::{Deserialize, Serialize};

use crate::database::models::article::TextType;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveOrEditText<'a> {
    /// This only needs to exist when editing an article.
    pub text_id: Option<i32>,
    pub text_type: TextType,
    pub title: &'a str,
    pub leading_paragraph: &'a str,
    pub text_body: &'a str,
    pub tags: &'a str,
    pub publish: Option<bool>,
    pub marked_as_done: bool,
}
