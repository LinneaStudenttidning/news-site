use rocket::{http::Status, serde::json::Json};

use crate::{data_dir, error::Error, token::Claims};

use self::requests::EditAboutUs;

use super::error_if_not_admin;

pub mod requests;

#[put("/site-settings/edit-about-us", data = "<data>")]
pub fn edit_about_us(claims: Claims, data: Json<EditAboutUs<'_>>) -> Result<String, Error> {
    error_if_not_admin(&claims)?;

    match data_dir::edit_about_us(data.about_us.to_string()) {
        true => Ok("Succesfully edited about us!".into()),
        false => Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Failed to update about us!",
            Status::InternalServerError,
        )),
    }
}
