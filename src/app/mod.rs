use crate::{
    anyresponder::AnyResponder,
    database::{
        models::{article::Text, creator::Creator},
        DatabaseHandler,
    },
    error::Error,
    token::Claims,
};
use rocket::{response::Redirect, Route, State};
use rocket_dyn_templates::{context, Template};

pub mod control_panel;
pub mod texts;

#[get("/")]
async fn landing(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let texts = Text::get_n_latest(db, 16, true).await?;

    Ok(Template::render(
        "landing",
        context! { tags, authors, texts },
    ))
}

#[get("/search?<q>")]
async fn search(q: Option<&str>, db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    let texts = Text::search(db, q.unwrap_or("")).await?;

    Ok(Template::render(
        "search",
        context! { texts, tags, authors, q },
    ))
}

#[get("/t/<id>/<title_slug>")]
async fn text_by_id(
    id: i32,
    title_slug: &str,
    db: &State<DatabaseHandler>,
    claims: Option<Claims>,
) -> Result<AnyResponder, Error> {
    let tags = Text::get_all_tags(db, None).await?;
    let authors = Creator::get_all_authors(db).await?;

    // Logged in users can view unpublished texts
    let is_logged_in = claims.is_some();
    let text = Text::get_by_id(db, id, !is_logged_in).await?;

    // If slug in url is incorrect, redirect to the correct one.
    if title_slug != text.title_slug {
        let redirect = Redirect::found(uri!(text_by_id(id, text.title_slug)));
        return Ok(AnyResponder::from(redirect));
    }

    // Render all the blocks in the article body.
    let mut rendered_blocks: Vec<String> = Vec::new();
    for block in text.text_body.iter() {
        rendered_blocks.push(
            block
                .render(db)
                .await // FIXME: A non-mutable solution might be more elegant, but this await keyword might make that difficult.
                .unwrap_or("INVALID BLOCK!".to_string()),
        );
    }

    // Bellow follows some bools used in the template to show different options/buttons.
    // FIXME: Maybe these can be generated in a more elegant way? E.g. as a struct generated from the claims?
    let can_edit_text = match &claims {
        Some(claims) => {
            claims.data.is_publisher() || (claims.sub == text.author && !text.is_published)
        }
        None => false,
    };
    let can_mark_as_done = match &claims {
        Some(claims) => !text.marked_as_done && claims.sub == text.author,
        None => false,
    };
    let can_unmark_as_done = match &claims {
        Some(claims) => !text.is_published && text.marked_as_done && claims.sub == text.author,
        None => false,
    };
    let can_publish_text = match &claims {
        Some(claims) => claims.data.is_publisher() && !text.is_published,
        None => false,
    };
    let can_unpublish_text = match &claims {
        Some(claims) => claims.data.is_publisher() && text.is_published,
        None => false,
    };

    let template = Template::render(
        "single-text-view",
        context! { text, rendered_blocks: rendered_blocks.join(""), tags, authors, is_logged_in, can_edit_text, can_mark_as_done, can_unmark_as_done, can_publish_text, can_unpublish_text },
    );
    Ok(AnyResponder::from(template))
}

#[get("/feed/atom.xml")]
async fn feed_atom(db: &State<DatabaseHandler>) -> Result<Template, Error> {
    let texts = Text::get_n_latest(db, 50, true).await?;
    let mut all_rendered_blocks: Vec<String> = Vec::new();

    for text in texts.iter() {
        let mut rendered_blocks: Vec<String> = Vec::new();
        for block in text.text_body.iter() {
            rendered_blocks.push(
                block
                    .render(db)
                    .await // FIXME: A non-mutable solution might be more elegant, but this await keyword might make that difficult.
                    .unwrap_or("INVALID BLOCK!".to_string()),
            );
        }
        all_rendered_blocks.push(rendered_blocks.join(""));
    }

    Ok(Template::render(
        "atom",
        context! { all_rendered_blocks, texts },
    ))
}

/// This should be mounted on `/`!
pub fn get_all_routes() -> Vec<Route> {
    routes![landing, search, text_by_id, feed_atom]
}
