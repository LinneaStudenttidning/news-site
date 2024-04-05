use crate::database::{models::article::Text, DatabaseHandler};
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};

pub mod control_panel;
pub mod texts;

#[get("/")]
async fn landing(db: &State<DatabaseHandler>) -> Result<Template, String> {
    let texts = Text::get_n_latest(db, 16)
        .await
        .map_err(|err| err.to_string())?;

    let tags = Text::get_all_tags(db)
        .await
        .map_err(|err| err.to_string())?;

    Ok(Template::render("landing", context! { texts, tags }))
}

#[get("/about-us")]
async fn about_us(db: &State<DatabaseHandler>) -> Result<Template, String> {
    let tags = Text::get_all_tags(db)
        .await
        .map_err(|err| err.to_string())?;

    Ok(Template::render("about", context! { tags }))
}

#[get("/t/<id>/<_title_slug>")]
async fn text_by_id(
    id: i32,
    _title_slug: &str,
    db: &State<DatabaseHandler>,
) -> Result<Template, String> {
    // FIXME: THIS MUST BE REMOVED IN PRODUCTION!
    Text::publish(db, id).await.map_err(|err| err.to_string())?;

    let text = Text::get_by_id(db, id)
        .await
        .map_err(|err| err.to_string())?;
    // TODO: Redirect if the url slug is incorrect, but ID is correct.
    println!("{:?}", text);

    let tags = Text::get_all_tags(db)
        .await
        .map_err(|err| err.to_string())?;

    Ok(Template::render("text-by-id", context! { text, tags }))
}

/// This should be mounted on `/`!
pub fn get_all_routes() -> Vec<Route> {
    routes![landing, about_us, text_by_id]
}
