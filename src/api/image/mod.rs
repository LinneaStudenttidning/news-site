use std::str::FromStr;

use image::ImageFormat;
use rocket::{form::Form, http::Status, response::Redirect, State};
use uuid::Uuid;

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

    // FIXME: Maybe this should yield an error as well?
    match Image::save_to_file(image.id, &form.image.data, image_format) {
        Ok(_) => (),
        Err(_) => {
            Image::delete(db, image.id).await.ok();
        }
    };

    Ok(Redirect::to("/control-panel/image-gallery"))
}

#[post("/image/delete/<id>")]
pub async fn image_delete(
    db: &State<DatabaseHandler>,
    claims: Claims,
    id: &str,
) -> Result<Redirect, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::promote_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    let id_as_uuid = Uuid::from_str(id)?;

    Image::delete(db, id_as_uuid).await?;

    Ok(Redirect::to("/control-panel/image-gallery"))
}
