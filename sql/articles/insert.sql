INSERT INTO
    articles (
        id,
        title,
        author,
        content,
        text_type,
        created_at,
        updated_at,
        tags
    )
VALUES
    (DEFAULT, $1, $2, $3, $4, DEFAULT, DEFAULT, $5)