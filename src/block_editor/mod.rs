use std::str::FromStr;

use comrak::{markdown_to_html, Options};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{models::image::Image, DatabaseHandler},
    error::Error,
};

/// Different colors that can be used for the text box.
/// These reflect the colors in our graphical profile.
///
/// The color names should be self explanatory.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TextBoxColor {
    Grey,
    Blue,
    Green,
    Red,
    Yellow,
}

impl TextBoxColor {
    fn as_str(&self) -> &'static str {
        match self {
            TextBoxColor::Grey => "grey",
            TextBoxColor::Blue => "blue",
            TextBoxColor::Green => "green",
            TextBoxColor::Red => "red",
            TextBoxColor::Yellow => "yellow",
        }
    }
}

/// These are all the different types of blocks supported by the block editor.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Block {
    /// A paragraph of text, it is stored as markdown, so formatting is possible.
    Paragraph { body_text: String },
    /// An image with a caption.
    Image {
        id: String,
        caption: String,
        image_data: Option<Image>,
    },
    /// A quote with a citation.
    Quote { quote: String, citation: String },
    /// Heading is a simple H2.
    Heading { heading: String },
    /// Raw html blocks. This should preferably be used as little as possible...
    RawHtml { html: String },
    /// Embeds a Youtube video.
    YouTube {
        video_link: String,
        caption: Option<String>,
    },
    /// A text box; a paragraph with a background color plate.
    TextBox {
        text: String,
        color: Option<TextBoxColor>,
    },
}

impl Block {
    pub async fn render(&self, db: &DatabaseHandler) -> Result<String, Error> {
        match self {
            Block::Heading { heading } => Ok(format!("<h2>{}</h2>", heading)),
            Block::Paragraph { body_text } => Ok(markdown_to_html(body_text, &Options::default())),
            Block::Quote { quote, citation } => Ok(format!(
                r#"<blockquote cite="{}">{}</blockquote>"#,
                citation, quote
            )),
            Block::Image {
                id,
                caption,
                image_data: _,
            } => {
                let image_id = Uuid::from_str(id)?;
                let image_data = Image::get_by_id(db, image_id).await?;

                Ok(format!(
                    r#"<img src="/dynamic-data/images/m/{}.webp" alt="{}" /><p class="caption">{} <span>Foto: {}.</span></p>"#,
                    image_data.id,
                    image_data.description.unwrap_or_default(),
                    caption,
                    image_data.author,
                ))
            }
            Block::RawHtml { html } => Ok(html.to_string()),
            Block::YouTube {
                video_link,
                caption,
            } => {
                let video_id_re = Regex::new(
                    r"(https://)?(youtu\.be|youtube\.com|www\.youtube\.com)/(watch\?v=|shorts|live)?\/?",
                )?;
                let video_id = video_id_re.replace(video_link, "");
                Ok(format!(
                    r#"<iframe class="youtube-video" src="https://www.youtube.com/embed/{}" title="YouTube video player" frameborder="0" allowfullscreen></iframe><p class="caption">{}</p>"#,
                    video_id,
                    caption.as_ref().unwrap_or(&"".to_string())
                ))
            }
            Block::TextBox { text, color } => Ok(format!(
                "<div class=\"textbox {}\">{}</div>",
                color.as_ref().map(|color| color.as_str()).unwrap_or(""),
                markdown_to_html(text, &Options::default())
            )),
        }
    }
}

// FIXME: This should be a trait, and all the different types should implement it.
// It will probably take some time to change it, but it's worth it to have a more modular way of adding modules.
// It would also (maybe?) make it possible to add in external modules for the block editor.

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the syntax of the `Block` enum and its variants is
    /// correct and can be serialized and deserialized correctly.
    #[test]
    fn test_syntax() {
        let article = [
            Block::Paragraph {
                body_text: "Hello, world!".to_string(),
            },
            Block::Image {
                id: "1".to_string(),
                caption: "Hello, world!".to_string(),
                image_data: None,
            },
            Block::Quote {
                quote: "Hello, world!".to_string(),
                citation: "Hello, world!".to_string(),
            },
        ];

        let article_json = serde_json::to_string(&article).unwrap();

        let article_parsed: Vec<Block> = serde_json::from_str(&article_json).unwrap();

        let expected_article_json = r#"[{"type":"Paragraph","body_text":"Hello, world!"},{"type":"Image","id":"1","caption":"Hello, world!","image_data":null},{"type":"Quote","quote":"Hello, world!","citation":"Hello, world!"}]"#;
        let expected_article_parsed = r#"[Paragraph { body_text: "Hello, world!" }, Image { id: "1", caption: "Hello, world!", image_data: None }, Quote { quote: "Hello, world!", citation: "Hello, world!" }]"#;

        let article_parsed_string = format!("{:?}", article_parsed);

        assert_eq!(article_json, expected_article_json);
        assert_eq!(article_parsed_string, expected_article_parsed);
    }
}
