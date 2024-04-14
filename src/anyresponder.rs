use rocket::response::{Redirect, Responder};
use rocket_dyn_templates::Template;

#[derive(Debug, Responder)]
pub enum AnyResponder {
    Template(Box<Template>),
    Redirect(Box<Redirect>),
}

impl From<Template> for AnyResponder {
    fn from(value: Template) -> Self {
        Self::Template(Box::new(value))
    }
}

impl From<Redirect> for AnyResponder {
    fn from(value: Redirect) -> Self {
        Self::Redirect(Box::new(value))
    }
}
