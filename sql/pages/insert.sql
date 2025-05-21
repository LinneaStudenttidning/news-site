INSERT INTO
    pages (
        path,
        title,
        text_body
    )
VALUES
    ($1, $2, $3)
RETURNING
    path,
    title,
    text_body AS "text_body!: Json<Vec<Block>>"
