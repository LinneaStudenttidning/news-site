use crate::database::{
    models::article::{Text, TextType},
    DatabaseHandler,
};
use rocket::{form::Form, response::Redirect, Route, State};
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn landing(db: &State<DatabaseHandler>) -> Result<Template, String> {
    let texts = Text::get_n_latest(db, 16)
        .await
        .map_err(|err| err.to_string())?;
    Ok(Template::render("landing", context! { texts }))
}

#[get("/about-us")]
fn about_us() -> Template {
    Template::render("about", context! {})
}

#[get("/editor")]
fn editor() -> Template {
    Template::render("editor", context! {})
}

#[get("/editor-v2")]
fn editor_v2() -> Template {
    Template::render("editor-v2", context! {})
}

#[get("/control-panel")]
fn control_panel() -> Template {
    Template::render("control_panel", context! {})
}

#[get("/text")]
fn text() -> Template {
    Template::render("text", context! {})
}

#[get("/text/<id>")]
async fn text_by_id(id: i32, db: &State<DatabaseHandler>) -> Result<Template, String> {
    Text::publish(db, id).await.map_err(|err| err.to_string())?;
    let text = Text::get_by_id(db, id)
        .await
        .map_err(|err| err.to_string())?;
    println!("{:?}", text);
    Ok(Template::render("text-by-id", context! { text }))
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

#[post("/publish-text", data = "<form>")]
async fn publish_text(form: Form<PublishTextForm<'_>>, db: &State<DatabaseHandler>) -> Redirect {
    let tags = form
        .tags
        .split(';')
        .map(String::from)
        .collect::<Vec<String>>();

    let text = Text::create(
        form.title,
        "UNKNOWN",
        form.leading_paragraph,
        form.text_body,
        form.text_type,
        tags,
    );

    match text.save_to_db(db).await {
        Ok(_) => Redirect::to("/"),
        Err(e) => {
            println!("{:?}", e);
            Redirect::to("/not-found")
        }
    }
}

pub fn get_all_routes() -> Vec<Route> {
    routes![
        landing,
        about_us,
        editor,
        editor_v2,
        control_panel,
        text,
        text_by_id,
        publish_text
    ]
}
