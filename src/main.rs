// Don't build with warnings if you used with feature flag "fail-on-warnings"
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
// This leads to false positives from clippy, if not allowed
// See: https://github.com/rust-lang/rust-clippy/issues/12642
// TL;DR: It breaks src/app/control_panel.rs at `struct PublishTextForm<'a>`
#![allow(clippy::blocks_in_conditions)]
// Forbid unsafe code; if, in the future, unsafe code is needed, then this may be removed.
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_i18n;

pub mod anyresponder;
pub mod api;
pub mod app;
pub mod block_editor;
pub mod database;
pub mod defaults;
pub mod error;
pub mod flash_msg;
pub mod token;

use std::{collections::HashMap, path::PathBuf, str::FromStr};

use comrak::{Options, markdown_to_html};
use database::{
    DatabaseHandler,
    models::{image::Image, page::Page},
};
use rocket::{
    Request, State,
    fs::FileServer,
    response::{Flash, Redirect},
};
use rocket_dyn_templates::{Engines, Template, context, tera};
use token::Claims;
use tokio::runtime::Runtime;
use uuid::Uuid;

// Load locales.
i18n!("locales", fallback = ["sv", "en"]);

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

    engines.tera.register_filter(
        "sanitize",
        |value: &tera::Value, _: &_| -> tera::Result<tera::Value> {
            let html = tera::from_value::<String>(value.clone())?;
            let sanitized_html = ammonia::clean(&html);
            Ok(tera::to_value(sanitized_html)?)
        },
    );

    engines.tera.register_function(
        "t",
        |value: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
            let gettext = value
                .get("t")
                .expect("Argument `t` (text entry) not defined!")
                .as_str()
                .ok_or("NOT A STRING!")
                .expect("NOT A STRING!");

            let default_locale = tera::to_value("sv")?;
            let locale = value
                .get("l")
                .unwrap_or(&default_locale)
                .as_str()
                .ok_or("NOT A STRING!")
                .expect("NOT A STRING!");

            let text = format!("{}", t!(&gettext, locale = &locale));

            Ok(tera::to_value(text)?)
        },
    );

    engines.tera.register_function(
        "image",
        |value: &HashMap<String, tera::Value>| -> tera::Result<tera::Value> {
            // Initialize the database connection.
            let database = match Runtime::new().unwrap().block_on(DatabaseHandler::create()) {
                Ok(db) => db,
                Err(err) => panic!(
                    "Encountered an error while connecting to database!\n{:?}",
                    err
                ),
            };

            let image_id = value
                .get("id")
                .expect("Argument `id` (image id) not defined!")
                .as_str()
                .ok_or("NOT A STRING!")
                .expect("NOT A STRING!");

            let caption = value
                .get("id")
                .expect("Argument `caption` (image caption) not defined!")
                .as_str()
                .ok_or("NOT A STRING!")
                .expect("NOT A STRING!");

            let image_uuid = Uuid::from_str(image_id).expect("Invalid UUID!");

            let image = Runtime::new()
                .unwrap()
                .block_on(Image::get_by_id(&database, image_uuid))
                .expect("Image not found!");

            let image_html = format!(r#"<img src="/dynamic-data/images/m/{}.webp" alt="{}"><p class="caption">{} <span>Foto: {}</span></p> | safe"#, image.id, image.description.unwrap_or_default(), caption, image.author);

            Ok(tera::to_value(image_html)?)
        },
    );
}

#[catch(404)]
async fn not_found() -> Template {
    let tags: Vec<String> = Vec::new();
    Template::render("errors/404", context! { tags })
}

#[catch(401)]
fn unauthorized(req: &Request) -> Flash<Redirect> {
    Flash::error(
        Redirect::to(format!("/control-panel/login?referer={}", req.uri())),
        "Du har ingen giltig session, var vänlig och logga in.",
    )
}

#[get("/<path..>", rank = 999)]
async fn page_finder(
    path: PathBuf,
    claims: Option<Claims>,
    db: &State<DatabaseHandler>,
) -> Option<Template> {
    println!("{}", path.to_str().unwrap());
    let page = match Page::get_by_path(db, path.to_str().unwrap()).await {
        Ok(page) => page,
        Err(_) => return None,
    };

    let mut rendered_blocks: Vec<String> = Vec::new();
    for block in page.text_body.iter() {
        rendered_blocks.push(
            block
                .render(db)
                .await
                .unwrap_or("INVALID BLOCK!".to_string()),
        );
    }

    let is_admin = match claims {
        Some(claims) => claims.data.is_publisher(),
        None => false,
    };

    Some(Template::render(
        "single-page-view",
        context! { rendered_blocks: rendered_blocks.join(""), page, is_admin },
    ))
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
        .mount("/", routes![page_finder])
        .register("/", catchers![not_found])
        .register("/", catchers![unauthorized])
        .launch()
        .await
    {
        Ok(_) => (),
        Err(err) => println!("Encountered an error while starting rocket:\n{}", err),
    }
}
