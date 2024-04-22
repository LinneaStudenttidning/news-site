use rocket::{form::Form, http::Status, response::Redirect, State};

use crate::{
    database::{models::article::Text, DatabaseHandler},
    error::Error,
    token::Claims,
};

use self::forms::{OnlyTextId, SaveOrEditText};

pub mod forms;

#[post("/text/save", data = "<form>")]
pub async fn text_save(
    form: Form<SaveOrEditText<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Redirect {
    let tags = match form.tags.is_empty() {
        true => Vec::new(),
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    // Only admins are allowed to publish on save.
    let should_publish_instantly = form.publish.unwrap_or(false) && claims.admin;

    let text = Text::create(
        form.title,
        &claims.data.username,
        form.leading_paragraph,
        form.text_body,
        form.text_type,
        tags,
        should_publish_instantly,
        form.marked_as_done,
    );

    match text.save_to_db(db).await {
        Ok(published_article) => Redirect::to(format!(
            "/t/{}/{}",
            published_article.id, published_article.title_slug
        )),
        Err(e) => {
            println!("{:?}", e);
            Redirect::to("/not-found")
        }
    }
}

#[post("/text/edit", data = "<form>")]
pub async fn text_edit(
    form: Form<SaveOrEditText<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, Error> {
    let tags = match form.tags.is_empty() {
        true => Vec::new(),
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
    };

    let text_id = match form.text_id {
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

    // Only admins are allowed to edit publish status.
    let should_publish = match claims.admin {
        true => form.publish.unwrap_or(false),
        false => current_text.is_published,
    };

    let updated_text = Text::update_by_id(
        db,
        text_id,
        form.title,
        form.leading_paragraph,
        form.text_body,
        form.text_type,
        &tags,
        should_publish,
        form.marked_as_done,
    )
    .await?;

    Ok(Redirect::to(format!(
        "/t/{}/{}",
        updated_text.id, updated_text.title_slug
    )))
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
