use forms::SaveOrEditPage;

use rocket::{http::Status, serde::json::Json, State};

use crate::{
    database::{models::page::Page, DatabaseHandler},
    error::Error,
    token::Claims,
};

use super::ReturnRedirect;

mod forms;

#[post("/page/save", data = "<data>")]
pub async fn page_save(
    data: Json<SaveOrEditPage<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Json<ReturnRedirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "You need to be an admin to access this view!",
            Status::Unauthorized,
        ));
    };

    if data.path.starts_with("/") {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Field `path` should not start with a slash!",
            Status::BadRequest,
        ));
    }

    let page = Page::create(data.path, data.title, data.blocks.clone());

    page.save_to_db(db).await.map(|page| {
        Json(ReturnRedirect {
            redirect: format!("/{}", page.path),
        })
    })
}

#[post("/page/edit", data = "<data>")]
pub async fn page_edit(
    data: Json<SaveOrEditPage<'_>>,
    db: &State<DatabaseHandler>,
    claims: Claims,
) -> Result<Json<ReturnRedirect>, Error> {
    if !claims.admin {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "You need to be an admin to access this view!",
            Status::Unauthorized,
        ));
    };

    if data.path.starts_with("/") {
        return Err(Error::create(
            &format!("{}:{}", file!(), line!()),
            "Field `path` should not start with a slash!",
            Status::BadRequest,
        ));
    }

    let old_path = match data.old_path {
        Some(text_id) => text_id,
        None => {
            return Err(Error::create(
                &format!("{}:{}", file!(), line!()),
                "Field `old_path` (`old_path`) not specified!",
                Status::BadRequest,
            ))
        }
    };

    let page = Page::update_by_path(
        db,
        old_path,
        data.path,
        data.title,
        sqlx::types::Json(data.blocks.clone()),
    );

    page.await.map(|page| {
        Json(ReturnRedirect {
            redirect: format!("/{}", page.path),
        })
    })
}
