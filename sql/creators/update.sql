UPDATE creators
SET
    display_name = $1,
    biography = $2,
    password = $3
WHERE
    username = $4
RETURNING
    display_name,
    username,
    password,
    biography,
    joined_at,
    role AS "role!: CreatorRole"
