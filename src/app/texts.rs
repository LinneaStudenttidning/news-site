use rocket::{Route, State};
use rocket_dyn_templates::{Template, context};

use crate::{
    database::{
        DatabaseHandler,
        models::{
            article::{Text, TextType},
            creator::Creator,
        },
    },
    error::Error,
};

#[get("/tag/<tag>")]
async fn texts_by_tag(tag: &str, db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let texts = Text::get_by_tag(db, tag).await?;

    Ok(Template::render(
        "landing",
        context! { texts, tags, authors, title: tag },
    ))
}

#[get("/type/<text_type>")]
async fn texts_by_type(
    text_type: TextType,
    db: &State<DatabaseHandler>,
) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let texts = Text::get_by_type(db, text_type).await?;

    Ok(Template::render(
        "landing",
        context! { texts, tags, authors, title: text_type },
    ))
}

#[get("/author/<author>")]
async fn texts_by_author(author: &str, db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let texts = Text::get_by_author(db, author, true).await?;
    let creator = Creator::get_by_username(db, author).await?;

    Ok(Template::render(
        "landing",
        context! { texts, tags, authors, title: creator.display_name },
    ))
}

/// These should be mounted on `/texts`!
pub fn get_all_routes() -> Vec<Route> {
    routes![texts_by_tag, texts_by_type, texts_by_author]
}
