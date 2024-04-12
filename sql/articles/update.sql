UPDATE articles SET
    title = $1,
    title_slug = $2,
    lead_paragraph = $3,
    text_body = $4,
    updated_at = NOW(),
    tags = $5,
    is_published = $6
WHERE
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
    is_published
