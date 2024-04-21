use rocket::{
    data::ToByteUnit,
    form::{DataField, FromFormField},
    http::ContentType,
};

#[derive(Debug, FromForm)]
pub struct NewCreator<'a> {
    pub username: &'a str,
    #[field(name = "display-name")]
    pub display_name: &'a str,
    pub password: &'a str,
    #[field(name = "as-publisher")]
    pub as_publisher: bool,
}

#[derive(Debug, FromForm)]
pub struct OnlyUsername<'a> {
    pub username: &'a str,
}

#[derive(Debug, FromForm)]
pub struct UpdateProfile<'a> {
    #[field(name = "display-name")]
    pub display_name: Option<&'a str>,
    pub biography: Option<&'a str>,
}

#[derive(Debug)]
pub struct File {
    pub content_type: ContentType,
    pub data: Vec<u8>,
}

#[rocket::async_trait]
impl<'a> FromFormField<'a> for File {
    /// https://stackoverflow.com/questions/73126693/retrieve-raw-file-content-from-form-data
    async fn from_data(field: DataField<'a, '_>) -> rocket::form::Result<'a, Self> {
        let stream = field.data.open(u32::MAX.bytes());
        let bytes = stream.into_bytes().await?;
        Ok(Self {
            content_type: field.content_type,
            data: bytes.value,
        })
    }
}

#[derive(Debug, FromForm)]
pub struct ImageOnly {
    pub image: File,
}
