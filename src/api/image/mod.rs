use std::str::FromStr;
use tokio::task;

use image::ImageFormat;
use rocket::{State, form::Form, http::Status, response::Redirect};
use uuid::Uuid;

use crate::{
    database::{DatabaseHandler, models::image::Image},
    error::Error,
    token::Claims,
};

use self::forms::UploadImage;

mod forms;

#[post("/image/upload", format = "multipart/form-data", data = "<form>")]
pub async fn image_upload(
    db: &State<DatabaseHandler>,
    form: Form<UploadImage<'_>>,
) -> Result<Redirect, Error> {
    let content_type = form.image.content_type.to_string();
    let image_format = ImageFormat::from_mime_type(content_type).ok_or(Error::create(
        &format!("{}:{}", file!(), line!()),
        "Sorry, failed to determine the mine-type of the image!",
        Status::InternalServerError,
    ))?;

    let tags = match form.tags.is_empty() {
        true => Vec::new(),
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    let image = Image::create(form.author, Some(form.description), tags);
    let image = image.save_to_db(db).await?;

    // Spin up a thread for saving the image
    let image_data = form.image.data.clone();
    let image_status =
        task::spawn_blocking(move || Image::save_to_file(image.id, &image_data, image_format))
            .await;

    if image_status.is_err() {
        Image::delete(db, image.id).await?;
        return Err(Error::create(
            "api::image::image_upload",
            format!(
                "Sorry, failed to save the image. Error: {:?}",
                image_status.err()
            )
            .as_str(),
            Status::InternalServerError,
        ));
    }

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
