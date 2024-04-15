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
    is_published,
    creators AS "creator!: Creator"
FROM
    articles
JOIN creators ON
    articles.author = creators.username
WHERE
    author = $1 AND is_published = true
