SELECT
    id,
    title,
    author,
    content,
    text_type AS "text_type!: TextType",
    created_at,
    updated_at,
    tags
FROM
    articles
WHERE
    $1 && tags AND is_published = true
