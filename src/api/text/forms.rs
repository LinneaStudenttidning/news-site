use serde::{Deserialize, Serialize};

use crate::{block_editor::Block, database::models::article::TextType};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct SaveOrEditText<'a> {
    /// This only needs to exist when editing an article.
    #[serde(rename = "text-id")]
    pub text_id: Option<i32>,
    #[serde(rename = "text-type")]
    pub text_type: TextType,
    pub title: &'a str,
    pub thumbnail: &'a str,
    #[serde(rename = "leading-paragraph")]
    pub leading_paragraph: &'a str,
    pub blocks: Vec<Block>,
    pub tags: &'a str,
}

#[derive(Debug, FromForm)]
pub struct OnlyTextId {
    #[field(name = "text-id")]
    pub text_id: i32,
}
