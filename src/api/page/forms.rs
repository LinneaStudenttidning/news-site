use serde::{Deserialize, Serialize};

use crate::block_editor::Block;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct SaveOrEditPage<'a> {
    #[serde(rename = "old-path")]
    pub old_path: Option<&'a str>,
    pub path: &'a str,
    pub title: &'a str,
    pub blocks: Vec<Block>,
}
