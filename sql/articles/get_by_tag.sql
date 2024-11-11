SELECT
    articles.id,
    title,
    title_slug,
    articles.author,
    thumbnail AS "thumbnail_id",
    lead_paragraph,
    text_body AS "text_body!: Json<Vec<Block>>",
    text_type AS "text_type!: TextType",
    articles.created_at,
    updated_at,
    articles.tags,
    is_published,
    marked_as_done,
    creators AS "creator!: Creator",
    images AS "thumbnail?: Image"
FROM
    articles
JOIN creators ON
    articles.author = creators.username
LEFT JOIN images ON
    articles.thumbnail = images.id
WHERE
    $1 = ANY(articles.tags) AND is_published = true
