UPDATE articles SET
    title = $1,
    title_slug = $2,
    thumbnail = $3,
    lead_paragraph = $4,
    text_body = $5,
    text_type = $6,
    updated_at = NOW(),
    tags = $7
FROM creators
WHERE
    articles.author = creators.username
AND
    id = $8
RETURNING
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
