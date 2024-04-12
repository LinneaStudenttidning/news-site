use crate::{
    app::rocket_uri_macro_text_by_id, database::models::creator::Creator, error::Error,
    token::Claims,
};
use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    request::FlashMessage,
    response::{Flash, Redirect},
    time::Duration,
    Route, State,
};
use rocket_dyn_templates::{context, Template};

use crate::database::{
    models::article::{Text, TextType},
    DatabaseHandler,
};

#[get("/")]
async fn control_panel(db: &State<DatabaseHandler>, claims: Claims) -> Result<Template, Error> {
    let creator = Creator::get_by_username(db, &claims.data.username).await?;
    let published_texts = Text::get_n_latest(db, 50, Some(true)).await?;
    let unpublished_texts = Text::get_n_latest(db, 50, Some(false)).await?;
    Ok(Template::render(
        "control_panel",
        context! { creator, published_texts, unpublished_texts },
    ))
}

#[get("/login")]
fn login_page(flash: Option<FlashMessage>) -> Template {
    Template::render(
        "login",
        context! { flash: flash.map(|msg| msg.message().to_string())},
    )
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

#[derive(FromForm)]
struct EditPasswordForm<'a> {
    current_password: &'a str,
    new_password: &'a str,
    confirm_new_password: &'a str,
}

#[post("/login", data = "<form>")]
async fn login(
    form: Form<LoginForm<'_>>,
    db: &State<DatabaseHandler>,
    jar: &CookieJar<'_>,
) -> Result<Flash<Redirect>, Error> {
    let creator = match Creator::get_by_username(db, form.username).await {
        Ok(creator) => creator,
        Err(_) => {
            return Ok(Flash::error(
                Redirect::to(uri!("/control-panel/login")),
                "Användaren finns inte!",
            ))
        }
    };

    let token = match creator.login(form.password).await {
        Ok(token) => token,
        Err(_) => {
            return Ok(Flash::error(
                Redirect::to(uri!("/control-panel/login")),
                "Fel lösenord",
            ))
        }
    };

    let cookie = Cookie::build(("token", token))
        .same_site(rocket::http::SameSite::Strict)
        .secure(true)
        .http_only(true)
        .max_age(Duration::hours(4));

    jar.add(cookie);

    Ok(Flash::success(Redirect::to("/control-panel"), ""))
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

    let _updated_creator = Creator::update_by_username(
        db,
        &claims.data.username,
        form.display_name,
        &creator.biography,
        &creator.password,
    )
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

    let _updated_creator = Creator::update_by_username(
        db,
        &claims.data.username,
        &creator.display_name,
        form.biography,
        &creator.password,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(Redirect::to("/control-panel"))
}

#[post("/change-password", data = "<form>")]
async fn change_password(
    form: Form<EditPasswordForm<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, Error> {
    let creator = Creator::get_by_username(db, &claims.data.username).await?;

    if form.new_password != form.confirm_new_password {
        return Err(Error::create(
            "Password check",
            "Password does not match!",
            Status::BadRequest,
        ));
    }

    if !Creator::verify_password(form.current_password, &creator.password).unwrap_or(false) {
        return Err(Error::create(
            "Password check",
            "Password is incorrect!",
            Status::BadRequest,
        ));
    }

    let _updated_creator = Creator::update_by_username(
        db,
        &claims.data.username,
        &creator.display_name,
        &creator.biography,
        &Creator::hash_password(form.new_password)?,
    )
    .await?;

    Ok(Redirect::to("/control-panel"))
}

#[get("/editor")]
fn editor() -> Template {
    Template::render("editor-v2", context! {})
}

#[get("/edit/<text_id>")]
async fn editor_text_id(text_id: i32, db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let text = Text::get_by_id(db, text_id, Some(false)).await?;

    Ok(Template::render("editor-v2", context! { text }))
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

#[derive(FromForm)]
struct EditTextForm<'a> {
    text_id: i32,
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
        true, // is_published
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

#[post("/edit-text", data = "<form>")]
async fn edit_text(form: Form<EditTextForm<'_>>, db: &State<DatabaseHandler>) -> Redirect {
    let tags = match form.tags.is_empty() {
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
        true => Vec::new(),
    };

    match Text::update_by_id(
        db,
        form.text_id,
        form.title,
        form.leading_paragraph,
        form.text_body,
        &tags,
        true, // is_published
    )
    .await
    {
        Ok(updated_article) => Redirect::to(uri!(text_by_id(
            updated_article.id,
            updated_article.title_slug
        ))),
        Err(e) => {
            println!("{:?}", e);
            Redirect::to("/not-found")
        }
    }
}

/// These should be mounted on `/control-panel`!
pub fn get_all_routes() -> Vec<Route> {
    routes![
        control_panel,
        login_page,
        login,
        change_display_name,
        change_biography,
        change_password,
        editor,
        editor_text_id,
        publish_text,
        edit_text
    ]
}
