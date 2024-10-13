use crate::database::models::article::TextType;

#[derive(Debug, FromForm)]
pub struct SaveOrEditText<'a> {
    /// This only needs to exist when editing an article.
    #[field(name = "text-id")]
    pub text_id: Option<i32>,
    #[field(name = "text-type")]
    pub text_type: TextType,
    pub title: &'a str,
    pub thumbnail: &'a str,
    #[field(name = "leading-paragraph")]
    pub leading_paragraph: &'a str,
    #[field(name = "text-body")]
    pub text_body: &'a str,
    pub tags: &'a str,
}

#[derive(Debug, FromForm)]
pub struct OnlyTextId {
    #[field(name = "text-id")]
    pub text_id: i32,
}
