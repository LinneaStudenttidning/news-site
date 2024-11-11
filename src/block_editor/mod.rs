use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{models::image::Image, DatabaseHandler},
    error::Error,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
enum Block<'a> {
    Paragraph {
        body_text: &'a str,
    },
    Image {
        id: &'a str,
        caption: &'a str,
        image_data: Option<Image>,
    },
    Quote {
        quote: &'a str,
        citation: &'a str,
    },
    Heading {
        heading: &'a str,
    },
    RawHtml {
        html: &'a str,
    },
}

impl Block<'_> {
    pub async fn _render(&self, db: &DatabaseHandler) -> Result<String, Error> {
        match self {
            Block::Heading { heading } => Ok(format!("<h2>{}</h2>", heading)),
            Block::Paragraph { body_text } => Ok(body_text.to_string()),
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
                    r#"<img src="/dynamic-data/images/m/{}.webp" alt="{}" /><p class="img-caption"><span>Foto: {}.</span> {}</p>"#,
                    image_data.id,
                    image_data.description.unwrap_or_default(),
                    image_data.author,
                    caption
                ))
            }
            Block::RawHtml { html } => Ok(html.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the syntax of the `Block` enum and its variants is
    /// correct and can be serialized and deserialized correctly.
    #[test]
    fn test_syntax() {
        let article = [
            Block::Paragraph {
                body_text: "Hello, world!",
            },
            Block::Image {
                id: "1",
                caption: "Hello, world!",
                image_data: None,
            },
            Block::Quote {
                quote: "Hello, world!",
                citation: "Hello, world!",
            },
        ];

        let article_json = serde_json::to_string(&article).unwrap();

        let article_parsed: Vec<Block> = serde_json::from_str(&article_json).unwrap();

        let expected_article_json = r#"[{"Paragraph":{"body_text":"Hello, world!"}},{"Image":{"id":"1","caption":"Hello, world!","image_data":null}},{"Quote":{"quote":"Hello, world!","citation":"Hello, world!"}}]"#;
        let expected_article_parsed = r#"[Paragraph { body_text: "Hello, world!" }, Image { id: "1", caption: "Hello, world!", image_data: None }, Quote { quote: "Hello, world!", citation: "Hello, world!" }]"#;

        let article_parsed_string = format!("{:?}", article_parsed);

        assert_eq!(article_json, expected_article_json);
        assert_eq!(article_parsed_string, expected_article_parsed);
    }
}
