SELECT
    path,
    title,
    text_body AS "text_body!: Json<Vec<Block>>"
FROM
    pages
WHERE
    path = $1
