use crate::{app::rocket_uri_macro_text_by_id, database::models::creator::Creator, token::Claims};
use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    response::Redirect,
    time::Duration,
    Route, State,
};
use rocket_dyn_templates::{context, Template};

use crate::database::{
    models::article::{Text, TextType},
    DatabaseHandler,
};

#[get("/")]
async fn control_panel(db: &State<DatabaseHandler>, claims: Claims) -> Result<Template, String> {
    let creator = Creator::get_by_username(db, &claims.data.username)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Template::render("control_panel", context! { creator }))
}

#[get("/login")]
fn login_page() -> Template {
    Template::render("login", context! {})
}

#[derive(FromForm)]
struct LoginForm<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(FromForm)]
struct EditDisplayNameForm<'a> {
    display_name: &'a str,
}

#[derive(FromForm)]
struct EditBiographyForm<'a> {
    biography: &'a str,
}

#[post("/login", data = "<form>")]
async fn login(
    form: Form<LoginForm<'_>>,
    db: &State<DatabaseHandler>,
    jar: &CookieJar<'_>,
) -> Result<Redirect, String> {
    let creator = Creator::get_by_username(db, form.username)
        .await
        .map_err(|_| "User does not exist!".to_string())?;

    let token = creator
        .login(form.password)
        .await
        .map_err(|_| "Wrong password!".to_string())?;

    let cookie = Cookie::build(("token", token))
        .same_site(rocket::http::SameSite::Strict)
        .secure(true)
        .http_only(true)
        .max_age(Duration::hours(4));

    jar.add(cookie);

    Ok(Redirect::to("/control-panel"))
}

#[post("/change-display-name", data = "<form>")]
async fn change_display_name(
    form: Form<EditDisplayNameForm<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, String> {
    let creator = Creator::get_by_username(db, &claims.data.username)
        .await
        .map_err(|_| "User does not exist!".to_string())?;

    let _updated_creator = Creator::update_by_username(db, &claims.data.username, form.display_name, &creator.biography)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Redirect::to("/control-panel"))
}

#[post("/change-biography", data = "<form>")]
async fn change_biography(
    form: Form<EditBiographyForm<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, String> {
    let creator = Creator::get_by_username(db, &claims.data.username)
        .await
        .map_err(|_| "User does not exist!".to_string())?;

    let _updated_creator = Creator::update_by_username(db, &claims.data.username, &creator.display_name, form.biography)
        .await
        .map_err(|e| e.to_string())?;

    Ok(Redirect::to("/control-panel"))
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
    routes![control_panel, login_page, login, change_display_name, change_biography, editor, publish_text]
}
