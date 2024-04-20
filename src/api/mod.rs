use rocket::Route;

use crate::api::text::{text_edit, text_save};

pub mod text;

pub fn get_all_routes() -> Vec<Route> {
    routes![
        // -> /text
        text_save, text_edit
    ]
}
