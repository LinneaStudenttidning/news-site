use crate::anyresponder::AnyResponder;
use crate::data_dir;
use crate::database::{models::article::Text, DatabaseHandler};
use crate::flash_msg::FlashMsg;
use crate::{database::models::creator::Creator, error::Error, token::Claims};
use rocket::request::FlashMessage;
use rocket::response::Redirect;
use rocket::{Route, State};
use rocket_dyn_templates::{context, Template};

#[get("/")]
async fn control_panel(
    db: &State<DatabaseHandler>,
    claims: Claims,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, Error> {
    let creator = Creator::get_by_username(db, &claims.data.username).await?;
    let published_texts = Text::get_by_author(db, &claims.data.username, true).await?;
    let unpublished_texts = Text::get_by_author(db, &claims.data.username, false).await?;
    let about_us = data_dir::get_about_us();

    let all_creator_usernames = Creator::get_all(db)
        .await?
        .into_iter()
        .map(|creator| creator.username)
        .collect::<Vec<String>>();

    let flash: FlashMsg = flash.into();

    Ok(Template::render(
        "control_panel/main",
        context! { creator, published_texts, unpublished_texts, all_creator_usernames, about_us, flash },
    ))
}

#[get("/login")]
fn login_page(flash: Option<FlashMessage>, claims: Option<Claims>) -> Result<AnyResponder, Error> {
    // Render template if logged out, else redirect to control panel
    if claims.is_none() {
        let flash: FlashMsg = flash.into();
        let template = Template::render("login", context! { flash });
        return Ok(AnyResponder::from(template));
    }
    let redirect = Redirect::found("/control-panel");
    Ok(AnyResponder::from(redirect))
}

#[get("/editor")]
fn editor(claims: Claims) -> Template {
    Template::render(
        "control_panel/editor_v2",
        context! { is_publisher: claims.data.is_publisher() },
    )
}

#[get("/edit/<text_id>")]
async fn editor_text_id(
    text_id: i32,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Template, Error> {
    let text = Text::get_by_id(db, text_id, Some(false)).await?;

    Ok(Template::render(
        "control_panel/editor_v2",
        context! { text, is_publisher: claims.data.is_publisher() },
    ))
}

/// These should be mounted on `/control-panel`!
pub fn get_all_routes() -> Vec<Route> {
    routes![control_panel, login_page, editor, editor_text_id,]
}
