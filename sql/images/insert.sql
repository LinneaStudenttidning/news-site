INSERT INTO
    images (
        id,
        author,
        description,
        created_at,
        tags
    )
VALUES
    ($1, $2, $3, DEFAULT, $4)