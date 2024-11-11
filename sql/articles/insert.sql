WITH inserted_article AS (
    INSERT INTO
        articles (
            id,
            title,
            title_slug,
            author,
            thumbnail,
            lead_paragraph,
            text_body,
            text_type,
            created_at,
            updated_at,
            tags,
            is_published,
            marked_as_done
        )
    VALUES
        (DEFAULT, $1, $2, $3, $4, $5, $6, $7, DEFAULT, DEFAULT, $8, $9, $10) RETURNING *
)
SELECT
    inserted_article.id,
    title,
    title_slug,
    inserted_article.author,
    thumbnail AS "thumbnail_id",
    lead_paragraph,
    text_body AS "text_body!: Json<Vec<Block>>",
    text_type AS "text_type!: TextType",
    inserted_article.created_at,
    updated_at,
    inserted_article.tags,
    is_published,
    marked_as_done,
    creators AS "creator!: Creator",
    images AS "thumbnail?: Image"
FROM inserted_article
JOIN creators ON
    inserted_article.author = creators.username
LEFT JOIN images ON
    inserted_article.thumbnail = images.id
