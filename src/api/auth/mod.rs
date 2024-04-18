use rocket::{
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    time::Duration,
    State,
};

use crate::{
    database::{models::creator::Creator, DatabaseHandler},
    error::Error,
    token::Claims,
};

use self::requests::{ChangePasswordOther, ChangePasswordSelf, Login};

use super::{default_response::DefaultResponse, error_if_not_admin};

pub mod requests;

#[post("/auth/login", data = "<data>")]
pub async fn login(
    data: Json<Login<'_>>,
    db: &State<DatabaseHandler>,
    jar: &CookieJar<'_>,
) -> DefaultResponse<Creator> {
    let creator = Creator::get_by_username(db, data.username).await?;
    let token = creator.login(data.password).await?;

    let cookie = Cookie::build(("token", token))
        .same_site(rocket::http::SameSite::Strict)
        .secure(true)
        .http_only(true)
        .max_age(Duration::hours(4));

    jar.add(cookie);

    Ok(Json(creator))
}

#[get("/auth/who")]
pub fn who(claims: Claims) -> Json<Creator> {
    Json(claims.data)
}

#[post("/auth/logout")]
pub fn logout(jar: &CookieJar<'_>) -> String {
    jar.remove("token");
    "Logged out successfully!".into()
}

#[post("/auth/change-password", data = "<data>")]
pub async fn change_password(
    claims: Claims,
    data: Json<ChangePasswordSelf<'_>>,
    db: &State<DatabaseHandler>,
) -> Result<String, Error> {
    let creator = Creator::get_by_username(db, &claims.data.username).await?;

    if data.new_password != data.repeat_new_password {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "New passwords must match!",
            Status::BadRequest,
        ));
    }

    if !Creator::verify_password(data.current_password, &creator.password)? {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Invalid current password!",
            Status::BadRequest,
        ));
    }

    Creator::change_password(db, &creator.username, data.new_password).await?;

    Ok("Successfully changed password!".into())
}

#[post("/auth/change-password-any", data = "<data>")]
pub async fn change_password_any(
    claims: Claims,
    data: Json<ChangePasswordOther<'_>>,
    db: &State<DatabaseHandler>,
) -> Result<String, Error> {
    error_if_not_admin(&claims)?;

    let creator = Creator::get_by_username(db, data.username).await?;
    Creator::change_password(db, &creator.username, data.new_password).await?;

    Ok(format!(
        "Successfully changed password of {} ({})!",
        creator.display_name, creator.username
    ))
}
