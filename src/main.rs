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

pub mod app;
pub mod database;
pub mod token;

use database::DatabaseHandler;
use rocket::fs::FileServer;
use rocket_dyn_templates::{context, tera, Engines, Template};

fn custom_tera(engines: &mut Engines) {
    engines.tera.register_filter(
        "markdown",
        |value: &tera::Value, _: &_| -> tera::Result<tera::Value> {
            let value = tera::from_value::<String>(value.clone())?;
            let value = markdown::to_html(&value);
            Ok(tera::to_value(value)?)
        },
    );
}

#[catch(404)]
async fn not_found() -> Template {
    let tags: Vec<String> = Vec::new();
    Template::render("errors/404", context! { tags })
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
        .mount("/texts", app::texts::get_all_routes())
        .mount("/control-panel", app::control_panel::get_all_routes())
        .mount("/static", FileServer::from("./static"))
        .register("/", catchers![not_found])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(err) => println!("Encountered an error while starting rocket:\n{}", err),
    }
}
