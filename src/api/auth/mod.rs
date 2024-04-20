use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    response::{Flash, Redirect},
    time::Duration,
    State,
};

use crate::{
    database::{models::creator::Creator, DatabaseHandler},
    error::Error,
    token::Claims,
};

use self::forms::{ChangePasswordOther, ChangePasswordSelf, Login};

mod forms;

#[post("/auth/login", data = "<form>")]
pub async fn auth_login(
    form: Form<Login<'_>>,
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
            "Detta konto är låst. Kontakta din ansvariga utgivare för att låsa upp kontot.",
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

#[post("/auth/logout")]
pub async fn auth_logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
    jar.remove("token");
    Flash::success(Redirect::to("/control-panel/login"), "Du är nu utloggad!")
}

#[post("/auth/change-password", data = "<form>")]
pub async fn auth_change_password(
    form: Form<ChangePasswordSelf<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Redirect, Error> {
    let creator = Creator::get_by_username(db, &claims.data.username).await?;

    if form.new_password != form.confirm_new_password {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Password does not match!",
            Status::BadRequest,
        ));
    }

    if !Creator::verify_password(form.current_password, &creator.password).unwrap_or(false) {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Password is incorrect!",
            Status::BadRequest,
        ));
    }

    Creator::change_password(db, &claims.data.username, form.new_password).await?;

    Ok(Redirect::to("/control-panel"))
}

#[post("/auth/change-password-other", data = "<form>")]
pub async fn auth_change_password_other(
    claims: Claims,
    db: &State<DatabaseHandler>,
    form: Form<ChangePasswordOther<'_>>,
) -> Result<Flash<Redirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        ));
    }

    let creator = Creator::get_by_username(db, form.username).await?;
    Creator::change_password(db, &creator.username, form.new_password).await?;

    Ok(Flash::success(
        Redirect::to("/control-panel"),
        format!(
            "Updaterad lösenordet för användaren: {} ({})",
            creator.username, creator.display_name
        ),
    ))
}
