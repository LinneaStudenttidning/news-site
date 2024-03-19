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

pub fn get_all_routes() -> Vec<Route> {
    routes![landing, about_us, editor]
}
