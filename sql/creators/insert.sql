INSERT INTO
    creators (
        display_name,
        username,
        password,
        biography,
        joined_at,
        role
    )
VALUES
    ($1, $2, $3, $4, DEFAULT, $5)
RETURNING
    display_name,
    username,
    password,
    biography,
    joined_at,
    role AS "role!: CreatorRole"
