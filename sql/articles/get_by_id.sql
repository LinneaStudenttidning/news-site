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
    tags,
    is_published
FROM
    articles
WHERE
    id = $1 AND is_published IN (true, $2)
