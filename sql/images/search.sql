SELECT
    id,
    author,
    description,
    created_at,
    tags
FROM
    images
WHERE
    to_tsquery($1) @@ search_vec
