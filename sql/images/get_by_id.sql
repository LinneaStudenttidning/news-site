SELECT
    id,
    author,
    description,
    created_at,
    tags
FROM
    images
WHERE
    id = $1
