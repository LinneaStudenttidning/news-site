use rocket::Route;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn landing() -> Template {
    Template::render("landing", context! {})
}

#[get("/about-us")]
fn about_us() -> Template {
    Template::render("about", context! {})
}

#[get("/editor")]
fn editor() -> Template {
    Template::render("editor", context! {})
}

#[get("/editor-v2")]
fn editor_v2() -> Template {
    Template::render("editor-v2", context! {})
}

#[get("/control-panel")]
fn control_panel() -> Template {
    Template::render("control_panel", context! {})
}

#[get("/text")]
fn text() -> Template {
    Template::render("text", context! {})
}

pub fn get_all_routes() -> Vec<Route> {
    routes![landing, about_us, editor, editor_v2, control_panel, text]
}
