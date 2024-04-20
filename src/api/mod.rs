use rocket::Route;

use crate::api::{
    auth::{auth_change_password, auth_change_password_other, auth_login, auth_logout},
    creator::{creator_demote, creator_lock, creator_new, creator_promote, creator_update_profile},
    site_settings::site_settings_update_about_us,
    text::{text_edit, text_save},
};

pub mod auth;
pub mod creator;
pub mod site_settings;
pub mod text;

pub fn get_all_routes() -> Vec<Route> {
    routes![
        // -> /auth
        auth_login,
        auth_logout,
        auth_change_password,
        auth_change_password_other,
        // -> /creator
        creator_new,
        creator_update_profile,
        creator_demote,
        creator_promote,
        creator_lock,
        // ->  /site-settings
        site_settings_update_about_us,
        // -> /text
        text_save,
        text_edit
    ]
}
