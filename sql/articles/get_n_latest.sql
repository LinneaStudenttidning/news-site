SELECT
    id,
    title,
    author,
    lead_paragraph,
    text_body,
    text_type AS "text_type!: TextType",
    created_at,
    updated_at,
    tags
FROM
    articles
ORDER BY
    created_at DESC
LIMIT
    $1
