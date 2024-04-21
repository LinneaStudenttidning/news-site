use std::{env::VarError, fmt};

use identicon_rs::error::IdenticonError;
use image::ImageError;
use rocket::{http::Status, Request, Response};
use rocket_dyn_templates::{context, Template};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub source: String,
    pub err_string: String,
    pub status: Status,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.source, self.err_string)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error {
            source: "SQLx Database error".to_string(),
            err_string: value.to_string(),
            status: Status::InternalServerError,
        }
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Error {
            source: value
                .source()
                .map(|source| source.to_string())
                .unwrap_or("Unknown source!".to_string()),
            err_string: value.to_string(),
            status: Status::InternalServerError,
        }
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Error {
            source: "Argon2".to_string(),
            err_string: value.to_string(),
            status: Status::InternalServerError,
        }
    }
}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Error {
            source: "VarError".to_string(),
            err_string: value.to_string(),
            status: Status::InternalServerError,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            source: "std::io::Error".to_string(),
            err_string: value.to_string(),
            status: Status::InternalServerError,
        }
    }
}

impl From<IdenticonError> for Error {
    fn from(value: IdenticonError) -> Self {
        Error {
            source: "identicon_rs::error::IdenticonError".to_string(),
            err_string: value.to_string(),
            status: Status::InternalServerError,
        }
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Error {
            source: "image::error::IdenticonError".to_string(),
            err_string: value.to_string(),
            status: Status::InternalServerError,
        }
    }
}

impl<'a> rocket::response::Responder<'a, 'static> for Error {
    fn respond_to(self, request: &'a Request<'_>) -> rocket::response::Result<'static> {
        // let headers = request
        //     .headers()
        //     .iter()
        //     .map(|h| (h.name().to_string(), h.value().to_string()))
        //     .collect::<Vec<(String, String)>>();

        Response::build_from(
            Template::render(
                "errors/generic",
                context! { error: self.to_string(), status: self.status, uri: request.uri()},
            )
            .respond_to(request)?,
        )
        .status(self.status)
        .ok()
    }
}

impl Error {
    pub fn create(source: &str, err_string: &str, status: Status) -> Self {
        Self {
            source: source.to_string(),
            err_string: err_string.to_string(),
            status,
        }
    }
}
