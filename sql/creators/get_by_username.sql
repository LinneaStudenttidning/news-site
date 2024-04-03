SELECT
    display_name,
    username,
    password,
    biography,
    joined_at,
    role AS "role!: CreatorRole"
FROM
    creators
WHERE
    username = $1
