use std::{env, fs};

use crate::{
    anyresponder::AnyResponder,
    database::{
        models::{article::Text, creator::Creator},
        DatabaseHandler,
    },
    defaults::DATA_DIR,
    error::Error,
    token::Claims,
};
use rocket::{response::Redirect, Route, State};
use rocket_dyn_templates::{context, Template};

pub mod control_panel;
pub mod texts;

#[get("/")]
async fn landing(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let texts = Text::get_n_latest(db, 16, true).await?;

    Ok(Template::render(
        "landing",
        context! { tags, authors, texts },
    ))
}

#[get("/about-us")]
async fn about_us(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let data_dir = env::var("DATA_DIR").unwrap_or(DATA_DIR.into());
    let about_us_md_path = format!("{data_dir}/about_us.md");
    let about_us_md = fs::read_to_string(about_us_md_path)?;

    Ok(Template::render(
        "about",
        context! { tags, authors, about_us_md },
    ))
}

#[get("/search?<q>")]
async fn search(q: Option<&str>, db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let texts = Text::search(db, q.unwrap_or("")).await?;

    Ok(Template::render(
        "search",
        context! { texts, tags, authors, q },
    ))
}

#[get("/t/<id>/<title_slug>")]
async fn text_by_id(
    id: i32,
    title_slug: &str,
    db: &State<DatabaseHandler>,
    claims: Option<Claims>,
) -> Result<AnyResponder, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let is_logged_in = match claims {
        Some(_creator) => true,
        None => false,
    };

    // Logged in users can view unpublished texts
    let text = Text::get_by_id(db, id, Some(!is_logged_in)).await?;

    if title_slug != text.title_slug {
        let redirect = Redirect::found(uri!(text_by_id(id, text.title_slug)));
        return Ok(AnyResponder::from(redirect));
    }

    let template = Template::render("text-by-id", context! { text, tags, authors, is_logged_in });
    Ok(AnyResponder::from(template))
}

#[get("/feed/atom.xml")]
async fn feed_atom(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let texts = Text::get_n_latest(db, 50, true).await?;

    Ok(Template::render("atom", context! { texts }))
}

/// This should be mounted on `/`!
pub fn get_all_routes() -> Vec<Route> {
    routes![landing, about_us, search, text_by_id, feed_atom]
}
