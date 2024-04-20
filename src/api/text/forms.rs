use crate::database::models::article::TextType;

#[derive(Debug, FromForm)]
pub struct SaveOrEditText<'a> {
    /// This only needs to exist when editing an article.
    #[field(name = "text-id")]
    pub text_id: Option<i32>,
    #[field(name = "text-type")]
    pub text_type: TextType,
    pub title: &'a str,
    #[field(name = "leading-paragraph")]
    pub leading_paragraph: &'a str,
    #[field(name = "text-body")]
    pub text_body: &'a str,
    pub tags: &'a str,
    pub publish: Option<bool>,
    #[field(name = "marked-as-done")]
    pub marked_as_done: bool,
}
