use image::ImageFormat;
use rocket::{
    form::Form,
    http::Status,
    response::{Flash, Redirect},
    State,
};

use crate::{
    database::{models::creator::Creator, DatabaseHandler},
    error::Error,
    token::Claims,
};

use self::forms::{ImageOnly, NewCreator, OnlyUsername, UpdateProfile};

pub mod forms;

#[post("/creator/new", data = "<form>")]
pub async fn creator_new(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<NewCreator<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    let creator = Creator::create(
        form.username,
        form.display_name,
        form.password,
        form.as_publisher,
    )?;

    let saved_creator = creator.save_to_db(db).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!(
            "Lyckades med att skapa ny användare: {} ({})",
            saved_creator.username, saved_creator.display_name
        ),
    ))
}

#[post("/creator/update-profile", data = "<form>")]
pub async fn creator_update_profile(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<UpdateProfile<'_>>,
) -> Result<Redirect, Error> {
    let creator = Creator::get_by_username(db, &claims.data.username).await?;

    // Default to current value if not specified in form.
    let display_name = form.display_name.unwrap_or(&creator.display_name);
    let biography = form.biography.unwrap_or(&creator.biography);

    Creator::update_by_username(db, &claims.data.username, display_name, biography).await?;

    Ok(Redirect::to("/control-panel"))
}

#[post(
    "/creator/update-profile-picture",
    format = "multipart/form-data",
    data = "<form>"
)]
pub async fn creator_update_profile_picture(
    claims: Claims,
    form: Form<ImageOnly>,
) -> Result<Redirect, Error> {
    let content_type = form.image.content_type.to_string();
    let image_format = ImageFormat::from_mime_type(content_type).ok_or(Error::create(
        &format!("{}:{}", file!(), line!()),
        "Sorry, the action you are performing requires admin access!",
        Status::Forbidden,
    ))?;

    Creator::change_profile_picture(&claims.data.username, &form.image.data, image_format)?;

    Ok(Redirect::to("/control-panel"))
}

#[post("/creator/promote", data = "<form>")]
pub async fn creator_promote(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<OnlyUsername<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::promote_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    Creator::promote(db, form.username).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!("Gjorde {} till ansvarig utgivare.", form.username),
    ))
}

#[post("/creator/demote", data = "<form>")]
pub async fn creator_demote(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<OnlyUsername<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::demote_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    if claims.sub == form.username {
        return Err(Error::create(
            "app::control_panel::demote_creator",
            "Sorry, you can't revoke your own admin access!",
            Status::BadRequest,
        ));
    }

    Creator::demote(db, form.username).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!("Tog bort {} som ansvarig utgivare.", form.username),
    ))
}

#[post("/creator/lock", data = "<form>")]
pub async fn creator_lock(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<OnlyUsername<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::lock_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    if claims.sub == form.username {
        return Err(Error::create(
            "app::control_panel::lock_creator",
            "Sorry, you can't lock your own account!",
            Status::BadRequest,
        ));
    }

    Creator::lock(db, form.username).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!("Användaren {} är nu låst.", form.username),
    ))
}
