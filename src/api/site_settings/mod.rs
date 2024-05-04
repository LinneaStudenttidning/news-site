use rocket::{
    form::Form,
    http::Status,
    response::{Flash, Redirect},
};

use crate::{data_dir, error::Error, token::Claims};

use self::forms::EditAboutUs;

mod forms;

#[post("/site-settings/update-about-us", data = "<form>")]
pub async fn site_settings_update_about_us(
    form: Form<EditAboutUs<'_>>,
    claims: Claims,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    match data_dir::edit_about_us(form.about_us.to_string()) {
        Ok(_) => Ok(Flash::success(
            Redirect::to("/control-panel"),
            "Lyckades med att ändra \"om oss\"".to_string(),
        )),
        Err(_) => Ok(Flash::error(
            Redirect::to("/control-panel"),
            "Misslyckades med att ändra \"om oss\"".to_string(),
        )),
    }
}
