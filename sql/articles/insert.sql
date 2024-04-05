INSERT INTO
    articles (
        id,
        is_published,
        title,
        title_slug,
        author,
        lead_paragraph,
        text_body,
        text_type,
        created_at,
        updated_at,
        tags
    )
VALUES
    (DEFAULT, DEFAULT, $1, $2, $3, $4, $5, $6, DEFAULT, DEFAULT, $7)
