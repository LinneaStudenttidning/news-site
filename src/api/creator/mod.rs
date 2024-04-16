use rocket::{serde::json::Json, State};

use crate::{
    database::{models::creator::Creator, DatabaseHandler},
    token::Claims,
};

use self::requests::NewCreator;

use super::{error_if_not_admin, DefaultResponse};

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
