WITH inserted_article AS (
    INSERT INTO
        articles (
            id,
            title,
            title_slug,
            author,
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
        (DEFAULT, $1, $2, $3, $4, $5, $6, DEFAULT, DEFAULT, $7, $8, $9) RETURNING *
) 
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
    marked_as_done,
    creators AS "creator!: Creator"
FROM inserted_article
JOIN creators ON
    inserted_article.author = creators.username
