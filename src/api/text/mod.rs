use rocket::{http::Status, serde::json::Json, State};

use crate::{
    database::{models::article::Text, DatabaseHandler},
    error::Error,
    token::Claims,
};

use self::requests::SaveOrEditText;

use super::DefaultResponse;

pub mod requests;

#[post("/text/save", data = "<data>")]
pub async fn text_save(
    data: Json<SaveOrEditText<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> DefaultResponse<Text> {
    let tags = match data.tags.is_empty() {
        true => Vec::new(),
        false => data
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    // Only admins are allowed to publish on save.
    let should_publish_instantly = data.publish.unwrap_or(false) && claims.admin;

    let text = Text::create(
        data.title,
        &claims.data.username,
        data.leading_paragraph,
        data.text_body,
        data.text_type,
        tags,
        should_publish_instantly,
        data.marked_as_done,
    );

    text.save_to_db(db).await.map(Json).map_err(Json)
}

#[put("/text/edit", data = "<data>")]
pub async fn text_edit(
    data: Json<SaveOrEditText<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> DefaultResponse<Text> {
    let tags = match data.tags.is_empty() {
        true => Vec::new(),
        false => data
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    let current_text_id = match data.text_id {
        Some(id) => id,
        None => {
            return Err(Json(Error::create(
                &format!("{}:{}", file!(), line!()),
                "Field `textId` (`text_id`) not specified!",
                Status::BadRequest,
            )))
        }
    };

    let current_text = Text::get_by_id(db, current_text_id, Some(false)).await?;

    // Only admins are allowed to publish on save.
    let should_publish_instantly = match claims.admin {
        true => data.publish.unwrap_or(current_text.is_published),
        false => current_text.is_published,
    };

    Text::update_by_id(
        db,
        current_text_id,
        data.title,
        data.leading_paragraph,
        data.text_body,
        &tags,
        should_publish_instantly,
    )
    .await
    .map(Json)
    .map_err(Json)
}
