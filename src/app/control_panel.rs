use crate::app::rocket_uri_macro_text_by_id;
use rocket::{form::Form, response::Redirect, Route, State};
use rocket_dyn_templates::{context, Template};

use crate::database::{
    models::article::{Text, TextType},
    DatabaseHandler,
};

#[get("/")]
fn control_panel() -> Template {
    Template::render("control_panel", context! {})
}

#[get("/editor")]
fn editor() -> Template {
    Template::render("editor-v2", context! {})
}

#[derive(FromForm)]
struct PublishTextForm<'a> {
    #[field(name = "text-type")]
    text_type: TextType,
    title: &'a str,
    #[field(name = "leading-paragraph")]
    leading_paragraph: &'a str,
    #[field(name = "text-body")]
    text_body: &'a str,
    tags: &'a str,
}

/// FIXME: THIS IS TEPORARY. MUST BE REMOVED / CHANGED BEFORE PRODUCTION.
#[post("/publish-text", data = "<form>")]
async fn publish_text(form: Form<PublishTextForm<'_>>, db: &State<DatabaseHandler>) -> Redirect {
    let tags = match form.tags.is_empty() {
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
        true => Vec::new(),
    };
    let text = Text::create(
        form.title,
        "UNKNOWN",
        form.leading_paragraph,
        form.text_body,
        form.text_type,
        tags,
    );
    match text.save_to_db(db).await {
        Ok(published_article) => Redirect::to(uri!(text_by_id(
            published_article.id,
            published_article.title_slug
        ))),
        Err(e) => {
            println!("{:?}", e);
            Redirect::to("/not-found")
        }
    }
}

/// These should be mounted on `/control-panel`!
pub fn get_all_routes() -> Vec<Route> {
    routes![control_panel, editor, publish_text]
}
