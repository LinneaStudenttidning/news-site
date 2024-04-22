UPDATE articles SET
    title = $1,
    title_slug = $2,
    lead_paragraph = $3,
    text_body = $4,
    text_type = $5,
    updated_at = NOW(),
    tags = $6
FROM creators
WHERE
    articles.author = creators.username
AND
    id = $7
RETURNING
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
    marked_as_done,
    creators AS "creator!: Creator"
