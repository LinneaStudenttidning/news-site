SELECT
    id,
    ts_headline(title, search_query, 'StartSel=<mark>, StopSel=</mark>') AS "title!",title
    title_slug,
    author,
    thumbnail,
    ts_headline(lead_paragraph, search_query, 'StartSel=<mark>, StopSel=</mark>') AS "lead_paragraph!",
    ts_headline(text_body, search_query, 'StartSel=<mark>, StopSel=</mark>') AS "text_body!",
    text_type AS "text_type!: TextType",
    created_at,
    updated_at,
    tags,
    is_published,
    marked_as_done,
    creators AS "creator!: Creator"
FROM
    to_tsquery(FORMAT('%s', ARRAY_TO_STRING(STRING_TO_ARRAY($1, ' '), ' & '))) AS search_query,
    articles
JOIN creators ON
    articles.author = creators.username
WHERE
    search_query @@ search_vec AND is_published = true
