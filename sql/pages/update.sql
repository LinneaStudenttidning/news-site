UPDATE pages
SET
    path = $2,
    title = $3,
    text_body = $4
WHERE
    path = $1
RETURNING
    path,
    title,
    text_body AS "text_body!: Json<Vec<Block>>"
