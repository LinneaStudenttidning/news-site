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
        is_published
    )
VALUES
    (DEFAULT, $1, $2, $3, $4, $5, $6, DEFAULT, DEFAULT, $7, $8) RETURNING
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
