UPDATE articles SET
    title = $1,
    title_slug = $2,
    thumbnail = $3,
    lead_paragraph = $4,
    text_body = $5,
    text_type = $6,
    updated_at = NOW(),
    tags = $7
FROM (
    SELECT
        articles.id,
        title,
        title_slug,
        articles.author,
        thumbnail,
        lead_paragraph,
        text_body,
        text_type,
        articles.created_at,
        updated_at,
        articles.tags,
        is_published,
        marked_as_done,
        creators,
        images
    FROM
        articles
    JOIN creators ON
        articles.author = creators.username
    LEFT JOIN images ON
        articles.thumbnail = images.id
    WHERE articles.id = $8
) AS updated_row
WHERE
    articles.id = $8
RETURNING
    updated_row.id,
    updated_row.title,
    updated_row.title_slug,
    updated_row.author,
    updated_row.thumbnail AS "thumbnail_id",
    updated_row.lead_paragraph,
    updated_row.text_body AS "text_body!: Json<Vec<Block>>",
    updated_row.text_type AS "text_type!: TextType",
    updated_row.created_at,
    updated_row.updated_at,
    updated_row.tags,
    updated_row.is_published,
    updated_row.marked_as_done,
    updated_row.creators AS "creator!: Creator",
    updated_row.images AS "thumbnail?: Image"
