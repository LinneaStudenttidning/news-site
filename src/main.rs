// Don't build with warnings.
#![deny(warnings)]
// This leads to false positives from clippy, if not allowed
// See: https://github.com/rust-lang/rust-clippy/issues/12642
// TL;DR: It breaks src/app/control_panel.rs at `struct PublishTextForm<'a>`
#![allow(clippy::blocks_in_conditions)]
// Forbid unsafe code; if, in the future, unsafe code is needed, then this may be removed.
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

pub mod anyresponder;
pub mod api;
pub mod app;
pub mod data_dir;
pub mod database;
pub mod defaults;
pub mod error;
pub mod flash_msg;
pub mod token;

use comrak::{markdown_to_html, Options};
use database::DatabaseHandler;
use rocket::{fs::FileServer, response::Flash, response::Redirect};
use rocket_dyn_templates::{context, tera, Engines, Template};

fn custom_tera(engines: &mut Engines) {
    engines.tera.register_filter(
        "markdown",
        |value: &tera::Value, _: &_| -> tera::Result<tera::Value> {
            let markdown = tera::from_value::<String>(value.clone())?;
            let raw_html = markdown_to_html(&markdown, &Options::default());
            let sanitized_html = ammonia::clean(&raw_html);
            Ok(tera::to_value(sanitized_html)?)
        },
    );
}

#[catch(404)]
async fn not_found() -> Template {
    let tags: Vec<String> = Vec::new();
    Template::render("errors/404", context! { tags })
}

#[catch(401)]
fn unauthorized() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/control-panel/login"),
        "Du har ingen giltig session, var vänlig och logga in.",
    )
}

#[rocket::main]
async fn main() {
    // Initialize the database connection.
    let database = match DatabaseHandler::create().await {
        Ok(db) => db,
        Err(err) => panic!(
            "Encountered an error while connecting to database!\n{:?}",
            err
        ),
    };

    // Launch the application
    match rocket::build()
        //.attach(Template::fairing())
        .attach(Template::custom(custom_tera))
        .manage(database)
        .mount("/", app::get_all_routes())
        .mount("/api", api::get_all_routes())
        .mount("/texts", app::texts::get_all_routes())
        .mount("/control-panel", app::control_panel::get_all_routes())
        .mount("/static", FileServer::from("./static"))
        .mount(
            "/dynamic-data/profile-pictures",
            FileServer::from("./data/profile-pictures"),
        )
        .mount("/dynamic-data/images", FileServer::from("./data/images"))
        .register("/", catchers![not_found])
        .register("/", catchers![unauthorized])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(err) => println!("Encountered an error while starting rocket:\n{}", err),
    }
}
