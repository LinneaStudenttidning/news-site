use crate::anyresponder::AnyResponder;
use crate::data_dir;
use crate::database::{models::article::Text, DatabaseHandler};
use crate::flash_msg::FlashMsg;
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

use self::form_structs::{
    ChangePasswordAnyForm, CreateCreatorForm, EditAboutUsForm, EditBiographyForm,
    EditDisplayNameForm, EditPasswordForm, EditTextForm, LockCreatorForm, LoginForm,
    PromoteDemoteForm, SaveTextForm,
};

pub mod form_structs;

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

#[post("/logout")]
async fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove("token");
    Flash::success(Redirect::to("/control-panel/login"), "Du är nu utloggad!")
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

    if creator.password == "LOCKED" {
        return Ok(Flash::error(
            Redirect::to(uri!("/control-panel/login")),
            "Detta konto är låst. Kontakta din ansvariga utgivare för att låsa kontot.",
        ));
    }

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

// PORTED
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

// PORTED
#[post("/change-about-us", data = "<form>")]
async fn change_about_us(
    form: Form<EditAboutUsForm<'_>>,
    claims: Claims,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::create_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    match data_dir::edit_about_us(form.about_us.to_string()) {
        true => Ok(Flash::success(
            Redirect::to("/control-panel"),
            "Lyckades med att ändra \"om oss\"".to_string(),
        )),
        false => Ok(Flash::error(
            Redirect::to("/control-panel"),
            "Misslyckades med att ändra \"om oss\"".to_string(),
        )),
    }
}

// PORTED
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

// PORTED
#[post("/save-text", data = "<form>")]
async fn publish_text(
    form: Form<SaveTextForm<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Redirect {
    let tags = match form.tags.is_empty() {
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
        true => Vec::new(),
    };

    // Only admins are allowed to publish on save.
    let publish = form.publish.unwrap_or(false) && claims.admin;

    let text = Text::create(
        form.title,
        &claims.data.username,
        form.leading_paragraph,
        form.text_body,
        form.text_type,
        tags,
        publish,
        form.marked_as_done,
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

// PORTED
#[post("/edit-text", data = "<form>")]
async fn edit_text(
    form: Form<EditTextForm<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, Error> {
    let tags = match form.tags.is_empty() {
        false => form
            .tags
            .split(';')
            .map(String::from)
            .collect::<Vec<String>>(),
        true => Vec::new(),
    };

    let current_text = Text::get_by_id(db, form.text_id, Some(false)).await?;

    // Only admins are allowed to change `is_published`.
    let input_publish = form.publish.unwrap_or(current_text.is_published);
    let publish = if claims.admin {
        input_publish
    } else {
        current_text.is_published
    };

    let updated_article = Text::update_by_id(
        db,
        form.text_id,
        form.title,
        form.leading_paragraph,
        form.text_body,
        &tags,
        publish,
    )
    .await?;

    Ok(Redirect::to(uri!(text_by_id(
        updated_article.id,
        updated_article.title_slug
    ))))
}

// PORTED
#[post("/create-creator", data = "<form>")]
async fn create_creator(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<CreateCreatorForm<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::create_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    let creator = Creator::create(
        form.username,
        form.display_name,
        form.password,
        form.as_publisher,
    )?;

    let saved_creator = creator.save_to_db(db).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!(
            "Lyckades med att skapa ny användare: {} ({})",
            saved_creator.username, saved_creator.display_name
        ),
    ))
}

#[post("/change-password-any", data = "<form>")]
async fn change_password_any(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<ChangePasswordAnyForm<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::create_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    let creator = Creator::get_by_username(db, form.username).await?;
    let new_password = Creator::hash_password(form.new_password)?;
    let updated_creator = Creator::update_by_username(
        db,
        &creator.username,
        &creator.display_name,
        &creator.biography,
        &new_password,
    )
    .await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!(
            "Updaterad lösenordet för användaren: {} ({})",
            updated_creator.username, updated_creator.display_name
        ),
    ))
}

// PORTED
#[post("/promote-creator", data = "<form>")]
async fn promote_creator(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<PromoteDemoteForm<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::promote_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    Creator::promote(db, form.username).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!("Gjorde {} till ansvarig utgivare.", form.username),
    ))
}

// PORTED
#[post("/demote-creator", data = "<form>")]
async fn demote_creator(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<PromoteDemoteForm<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::demote_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    if claims.sub == form.username {
        return Err(Error::create(
            "app::control_panel::demote_creator",
            "Sorry, you can't revoke your own admin access!",
            Status::BadRequest,
        ));
    }

    Creator::demote(db, form.username).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!("Tog bort {} som ansvarig utgivare.", form.username),
    ))
}

// PORTED
#[post("/lock-creator", data = "<form>")]
async fn lock_creator(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<LockCreatorForm<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            "app::control_panel::lock_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    if claims.sub == form.username {
        return Err(Error::create(
            "app::control_panel::lock_creator",
            "Sorry, you can't lock your own account!",
            Status::BadRequest,
        ));
    }

    Creator::lock(db, form.username).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!("Användaren {} är nu låst.", form.username),
    ))
}

/// These should be mounted on `/control-panel`!
pub fn get_all_routes() -> Vec<Route> {
    routes![
        control_panel,
        login_page,
        login,
        logout,
        change_display_name,
        change_biography,
        change_password,
        editor,
        editor_text_id,
        publish_text,
        edit_text,
        create_creator,
        change_password_any,
        promote_creator,
        change_about_us,
        demote_creator,
        lock_creator,
    ]
}
