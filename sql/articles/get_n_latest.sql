SELECT
    id,
    title,
    title_slug,
    author,
    lead_paragraph,
    text_body,
    text_type AS "text_type!: TextType",
    created_at,
    updated_at,
    tags
FROM
    articles
WHERE is_published = $1
ORDER BY
    created_at DESC
LIMIT
    $2
