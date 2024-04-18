use rocket::serde::json::Json;

use crate::error::Error;

pub type DefaultResponse<T> = Result<Json<T>, Json<Error>>;
pub fn default_response<T>(result: Result<T, Error>) -> DefaultResponse<T> {
    result.map(Json).map_err(Json)
}
