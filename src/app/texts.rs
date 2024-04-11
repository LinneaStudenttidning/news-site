use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};

use crate::{
    database::{
        models::article::{Text, TextType},
        DatabaseHandler,
    },
    error::Error,
};

#[get("/tag/<tag>")]
async fn texts_by_tag(tag: &str, db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let texts = Text::get_by_tag(db, tag).await?;

    let tags = Text::get_all_tags(db).await?;

    Ok(Template::render("landing", context! { texts, tags }))
}

#[get("/type/<text_type>")]
async fn texts_by_type(
    text_type: TextType,
    db: &State<DatabaseHandler>,
) -> Result<Template, Error> {
    let texts = Text::get_by_type(db, text_type).await?;

    let tags = Text::get_all_tags(db).await?;

    Ok(Template::render("landing", context! { texts, tags }))
}

#[get("/author/<author>")]
async fn texts_by_author(author: &str, db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let texts = Text::get_by_author(db, author).await?;

    let tags = Text::get_all_tags(db).await?;

    Ok(Template::render("landing", context! { texts, tags }))
}

/// These should be mounted on `/texts`!
pub fn get_all_routes() -> Vec<Route> {
    routes![texts_by_tag, texts_by_type, texts_by_author]
}
