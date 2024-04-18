use rocket::{http::Status, Route};

use crate::{
    api::{
        auth::{change_password, change_password_any, login, logout},
        creator::{creator_new, demote, promote},
        text::{text_edit, text_save},
    },
    error::Error,
    token::Claims,
};

pub mod auth;
pub mod creator;
pub mod default_response;
pub mod text;

pub fn error_if_not_admin(claims: &Claims) -> Result<(), Error> {
    match claims.admin {
        true => Ok(()),
        false => Err(Error::create(
            "app::control_panel::create_creator",
            "Sorry, the action you are performing requires admin access!",
            Status::Forbidden,
        )),
    }
}

/// Gets all API routes. Should be mounted at `/api`.
pub fn get_all_routes() -> Vec<Route> {
    routes![
        // * /auth
        login,
        logout,
        change_password,
        change_password_any,
        // * /creator
        creator_new,
        demote,
        promote,
        // * /text
        text_save,
        text_edit
    ]
}
