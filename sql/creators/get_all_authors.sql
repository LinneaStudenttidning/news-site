SELECT
    display_name,
    username,
    password,
    biography,
    joined_at,
    role AS "role!: CreatorRole"
FROM (
    SELECT author, COUNT(*) as total_articles
    FROM
        articles
    WHERE
        is_published = true
    GROUP BY author
) JOIN creators ON author = creators.username
ORDER BY total_articles DESC
