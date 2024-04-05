// Don't build with warnings.
#![deny(warnings)]

#[macro_use]
extern crate rocket;

pub mod app;
pub mod database;
pub mod token;

use database::DatabaseHandler;
use rocket::fs::FileServer;
use rocket_dyn_templates::{context, tera, Template};

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
        .attach(Template::custom(|engines|{
            engines.tera.register_filter(
                "markdown",
                |value: &tera::Value, _: &_| -> tera::Result<tera::Value> {
                    let value = tera::from_value::<String>(value.clone())?;
                    let value = markdown::to_html(&value);
                    Ok(tera::to_value(value)?)
                },
            );
        }))
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
