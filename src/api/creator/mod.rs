use rocket::{http::Status, serde::json::Json, State};

use crate::{
    database::{models::creator::Creator, DatabaseHandler},
    error::Error,
    token::Claims,
};

use self::requests::{NewCreator, OnlyUsername, UpdateProfile};

use super::{
    default_response::{default_response, DefaultResponse},
    error_if_not_admin,
};

pub mod requests;

#[post("/creator/new", data = "<data>")]
pub async fn creator_new(
    claims: Claims,
    data: Json<NewCreator<'_>>,
    db: &State<DatabaseHandler>,
) -> DefaultResponse<Creator> {
    error_if_not_admin(&claims)?;

    let creator = Creator::create(
        data.username,
        data.display_name,
        data.password,
        data.as_publisher,
    )?;

    let saved_creator = creator.save_to_db(db).await?;

    Ok(Json(saved_creator))
}

#[put("/creator/demote", data = "<data>")]
pub async fn demote(
    claims: Claims,
    data: Json<OnlyUsername<'_>>,
    db: &State<DatabaseHandler>,
) -> Result<String, Error> {
    error_if_not_admin(&claims)?;

    if claims.data.username == data.username {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Sorry, you can't revoke your own admin access!",
            Status::BadRequest,
        ));
    }

    Creator::demote(db, data.username).await?;
    Ok(format!("Succesfully demoted {}!", data.username))
}

#[put("/creator/promote", data = "<data>")]
pub async fn promote(
    claims: Claims,
    data: Json<OnlyUsername<'_>>,
    db: &State<DatabaseHandler>,
) -> Result<String, Error> {
    error_if_not_admin(&claims)?;
    Creator::promote(db, data.username).await?;
    Ok(format!("Succesfully promoted {}!", data.username))
}

#[put("/creator/update-profile", data = "<data>")]
pub async fn update_profile(
    claims: Claims,
    data: Json<UpdateProfile<'_>>,
    db: &State<DatabaseHandler>,
) -> DefaultResponse<Creator> {
    let creator = Creator::get_by_username(db, &claims.sub).await?;

    let display_name = data.display_name.unwrap_or(&creator.display_name);
    let biography = data.biography.unwrap_or(&creator.biography);

    default_response(
        Creator::update_by_username(db, &claims.sub, display_name, biography, &creator.password)
            .await,
    )
}

#[put("/creator/lock", data = "<data>")]
pub async fn lock_creator(
    claims: Claims,
    data: Json<OnlyUsername<'_>>,
    db: &State<DatabaseHandler>,
) -> DefaultResponse<Creator> {
    error_if_not_admin(&claims)?;

    if claims.sub == data.username {
        return default_response(Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "You can't lock your own account!",
            Status::BadRequest,
        )));
    }

    Creator::lock(db, data.username).await?;
    let locked_creator = Creator::get_by_username(db, data.username).await?;

    default_response(Ok(locked_creator))
}
