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
    text_type = $1 AND is_published = true
