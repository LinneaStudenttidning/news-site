SELECT
    id,
    title,
    title_slug,
    author,
    thumbnail,
    lead_paragraph,
    text_body,
    text_type AS "text_type!: TextType",
    created_at,
    updated_at,
    tags,
    is_published,
    marked_as_done,
    creators AS "creator!: Creator"
FROM
    articles
JOIN creators ON
    articles.author = creators.username
WHERE is_published = $1
ORDER BY
    created_at DESC
LIMIT
    $2
