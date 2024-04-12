SELECT
    id,
    title,
    title_slug,
    author,
    lead_paragraph,
    text_body,
    text_type AS "text_type!: TextType",
    created_at,
    updated_at,
    tags,
    is_published
FROM
    articles
WHERE
    to_tsquery($1) @@ search_vec AND is_published = true
