use std::str::FromStr;

use rocket::{form::Form, http::Status, response::Redirect, serde::json::Json, State};
use uuid::Uuid;

use crate::{
    database::{
        models::{article::Text, image::Image},
        DatabaseHandler,
    },
    error::Error,
    token::Claims,
};

use self::forms::{OnlyTextId, SaveOrEditText};

use super::ReturnRedirect;

pub mod forms;

#[post("/text/save", data = "<data>")]
pub async fn text_save(
    data: Json<SaveOrEditText<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Json<ReturnRedirect>, Error> {
    let tags = match data.tags.is_empty() {
        true => Vec::new(),
        false => data
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    println!("{:?}", data.blocks);

    // TODO: check if its valid
    // let parsed_body_json: Vec<Block> = serde_json::from_str(form.text_body).unwrap();

    let text = Text::create(
        data.title,
        &claims.data.username,
        data.leading_paragraph,
        data.blocks.clone(),
        data.text_type,
        tags,
    );

    text.save_to_db(db).await.map(|text| {
        Json(ReturnRedirect {
            redirect: format!("/t/{}/{}", text.id, text.title_slug),
        })
    })
}

#[post("/text/edit", format = "json", data = "<data>")]
pub async fn text_edit(
    data: Json<SaveOrEditText<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Json<ReturnRedirect>, Error> {
    let tags = match data.tags.is_empty() {
        true => Vec::new(),
        false => data
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    let text_id = match data.text_id {
        Some(text_id) => text_id,
        None => {
            return Err(Error::create(
                &format!("{}:{}", file!(), line!()),
                "Field `text-id` (`text_id`) not specified!",
                Status::BadRequest,
            ))
        }
    };

    let current_text = Text::get_by_id(db, text_id, false).await?;

    if current_text.author != claims.sub && !claims.admin {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Must be owner of text or publisher to edit!",
            Status::Unauthorized,
        ));
    }

    if current_text.is_published && !claims.admin {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Cannot edit published text if not publisher!",
            Status::Unauthorized,
        ));
    }

    let updated_text = Text::update_by_id(
        db,
        text_id,
        data.title,
        match Uuid::from_str(data.thumbnail) {
            Ok(thumbnail_id) => Image::get_by_id(db, thumbnail_id)
                .await
                .ok()
                .map(|image| image.id),
            _ => None,
        },
        data.leading_paragraph,
        sqlx::types::Json(data.blocks.clone()),
        data.text_type,
        &tags,
    )
    .await?;

    Ok(Json(ReturnRedirect {
        redirect: format!("/t/{}/{}", updated_text.id, updated_text.title_slug),
    }))
}

#[post("/text/set-publish-status/<publish_status>", data = "<form>")]
pub async fn text_set_publish_status(
    form: Form<OnlyTextId>,
    publish_status: bool,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, Error> {
    let text = Text::get_by_id(db, form.text_id, false).await?;
    Text::set_publish_status(db, &claims.data, form.text_id, publish_status)
        .await
        .map(|_| Redirect::to(format!("/t/{}/{}", text.id, text.title_slug)))
}

#[post("/text/set-done-status/<done_status>", data = "<form>")]
pub async fn text_set_done_status(
    form: Form<OnlyTextId>,
    done_status: bool,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, Error> {
    let text = Text::get_by_id(db, form.text_id, false).await?;
    Text::set_done_status(db, &claims.data, form.text_id, done_status)
        .await
        .map(|_| Redirect::to(format!("/t/{}/{}", text.id, text.title_slug)))
}
