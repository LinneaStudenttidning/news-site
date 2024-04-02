INSERT INTO
    articles (
        id,
        is_published,
        title,
        author,
        content,
        text_type,
        created_at,
        updated_at,
        tags
    )
VALUES
    (DEFAULT, DEFAULT, $1, $2, $3, $4, DEFAULT, DEFAULT, $5)