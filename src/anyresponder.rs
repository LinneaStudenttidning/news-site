use rocket::response::{Redirect, Responder};
use rocket_dyn_templates::Template;

#[derive(Debug, Responder)]
pub enum AnyResponder {
    Template(Box<Template>),
    Redirect(Box<Redirect>),
}
