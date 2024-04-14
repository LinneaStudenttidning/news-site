use crate::database::models::article::TextType;

#[derive(FromForm)]
pub struct PublishTextForm<'a> {
    #[field(name = "text-type")]
    pub text_type: TextType,
    pub title: &'a str,
    #[field(name = "leading-paragraph")]
    pub leading_paragraph: &'a str,
    #[field(name = "text-body")]
    pub text_body: &'a str,
    pub tags: &'a str,
}

#[derive(FromForm)]
pub struct EditTextForm<'a> {
    pub text_id: i32,
    pub title: &'a str,
    #[field(name = "leading-paragraph")]
    pub leading_paragraph: &'a str,
    #[field(name = "text-body")]
    pub text_body: &'a str,
    pub tags: &'a str,
}

#[derive(FromForm)]
pub struct LoginForm<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(FromForm)]
pub struct EditBiographyForm<'a> {
    pub biography: &'a str,
}

#[derive(FromForm)]
pub struct EditPasswordForm<'a> {
    pub current_password: &'a str,
    pub new_password: &'a str,
    pub confirm_new_password: &'a str,
}

#[derive(FromForm)]
pub struct EditDisplayNameForm<'a> {
    pub display_name: &'a str,
}
