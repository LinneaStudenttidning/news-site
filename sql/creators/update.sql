UPDATE creators
SET
    display_name = $1,
    biography = $2
WHERE
    username = $3
RETURNING
    display_name,
    username,
    password,
    biography,
    joined_at,
    role AS "role!: CreatorRole"
