SELECT
    id,
    author,
    ts_headline(description, search_query, 'StartSel=<mark>, StopSel=</mark>') AS "description!",
    created_at,
    tags
FROM
    to_tsquery(FORMAT('%s', ARRAY_TO_STRING(STRING_TO_ARRAY($1, ' '), ' & '))) AS search_query,
    images
WHERE
    search_query @@ search_vec
