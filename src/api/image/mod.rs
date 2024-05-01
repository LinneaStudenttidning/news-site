use image::ImageFormat;
use rocket::{form::Form, http::Status, response::Redirect, State};

use crate::{
    database::{models::image::Image, DatabaseHandler},
    error::Error,
    token::Claims,
};

use self::forms::UploadImage;

mod forms;

#[post("/image/upload", format = "multipart/form-data", data = "<form>")]
pub async fn image_upload(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<UploadImage<'_>>,
) -> Result<Redirect, Error> {
    let content_type = form.image.content_type.to_string();
    let image_format = ImageFormat::from_mime_type(content_type).ok_or(Error::create(
        &format!("{}:{}", file!(), line!()),
        "Sorry, the action you are performing requires admin access!",
        Status::Forbidden,
    ))?;

    let tags = match form.tags.is_empty() {
        true => Vec::new(),
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    let image = Image::create(&claims.sub, Some(form.description), tags);
    let image = image.save_to_db(db).await?;

    // FIXME: If this fails it should delete the database entry!
    Image::save_to_file(image.id, &form.image.data, image_format)?;

    Ok(Redirect::to("/dynamic-data/images/m/{{ image.id }}.webp"))
}
