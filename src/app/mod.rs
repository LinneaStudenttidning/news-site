use crate::{
    anyresponder::AnyResponder,
    database::{models::article::Text, DatabaseHandler},
    error::Error,
};
use rocket::{response::Redirect, Route, State};
use rocket_dyn_templates::{context, Template};

pub mod control_panel;
pub mod texts;

#[get("/")]
async fn landing(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let texts = Text::get_n_latest(db, 16, Some(true)).await?;

    let tags = Text::get_all_tags(db, None).await?;

    Ok(Template::render("landing", context! { texts, tags }))
}

#[get("/about-us")]
async fn about_us(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;

    Ok(Template::render("about", context! { tags }))
}

// FIXME: Refactor this mess...
#[get("/t/<id>/<title_slug>")]
async fn text_by_id(
    id: i32,
    title_slug: &str,
    db: &State<DatabaseHandler>,
) -> Result<AnyResponder, Error> {
    let text = Text::get_by_id(db, id, None).await?;

    if title_slug != text.title_slug {
        let redirect = AnyResponder::Redirect(Box::new(Redirect::found(uri!(text_by_id(
            id,
            text.title_slug
        )))));
        return Ok(redirect);
    }

    let tags = Text::get_all_tags(db, None).await?;

    let template = AnyResponder::Template(Box::new(Template::render(
        "text-by-id",
        context! { text, tags },
    )));
    Ok(template)
}

#[get("/feed/atom.xml")]
async fn feed_atom(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let texts = Text::get_n_latest(db, 50, Some(true)).await?;

    Ok(Template::render("atom", context! { texts }))
}

/// This should be mounted on `/`!
pub fn get_all_routes() -> Vec<Route> {
    routes![landing, about_us, text_by_id, feed_atom]
}
