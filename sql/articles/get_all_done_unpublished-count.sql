SELECT
    COUNT(*) AS "count!: i64"
FROM
    articles
WHERE
    marked_as_done = true AND is_published = false
