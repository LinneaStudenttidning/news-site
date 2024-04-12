use crate::{
    database::{models::article::Text, DatabaseHandler},
    error::Error,
};
use rocket::{Route, State};
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

#[get("/t/<id>/<_title_slug>")]
async fn text_by_id(
    id: i32,
    _title_slug: &str,
    db: &State<DatabaseHandler>,
) -> Result<Template, Error> {
    let text = Text::get_by_id(db, id, None).await?;
    // TODO: Redirect if the url slug is incorrect, but ID is correct.
    println!("{:?}", text);

    let tags = Text::get_all_tags(db, None).await?;

    Ok(Template::render("text-by-id", context! { text, tags }))
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
