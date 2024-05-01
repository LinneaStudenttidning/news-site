use rocket::{
    data::ToByteUnit,
    form::{DataField, FromFormField},
    http::ContentType,
};

/// TODO: This is duplicated from the form for uploading profile picture!
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
pub struct UploadImage<'a> {
    pub description: &'a str,
    pub image: File,
    pub tags: &'a str,
}
