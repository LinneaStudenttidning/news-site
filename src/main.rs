#[macro_use]
extern crate rocket;

pub mod app;
pub mod database;

use app::get_all_routes;
use database::DatabaseHandler;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[rocket::main]
async fn main() {
    let database = match DatabaseHandler::create().await {
        Ok(db) => db,
        Err(err) => panic!(
            "Encountered an error while connecting to database!\n{:?}",
            err
        ),
    };

    match rocket::build()
        .attach(Template::fairing())
        .manage(database)
        .mount("/", get_all_routes())
        .mount("/static", FileServer::from("./static"))
        .launch()
        .await
    {
        Ok(_) => (),
        Err(err) => println!("Encountered an error while starting rocket:\n{}", err),
    }
}
