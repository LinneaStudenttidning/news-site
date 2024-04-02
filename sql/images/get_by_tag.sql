SELECT
    id,
    author,
    description,
    created_at,
    tags
FROM
    images
WHERE
    $1 = ANY(tags)