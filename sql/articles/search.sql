SELECT
    articles.id,
    ts_headline(title, search_query, 'StartSel=<mark>, StopSel=</mark>') AS "title!",title
    title_slug,
    articles.author,
    thumbnail AS "thumbnail_id",
    ts_headline(lead_paragraph, search_query, 'StartSel=<mark>, StopSel=</mark>') AS "lead_paragraph!",
    ts_headline(text_body, search_query, 'StartSel=<mark>, StopSel=</mark>') AS "text_body!: Json<Vec<Block>>",
    text_type AS "text_type!: TextType",
    articles.created_at,
    updated_at,
    articles.tags,
    is_published,
    marked_as_done,
    creators AS "creator!: Creator",
    images AS "thumbnail?: Image"
FROM
    to_tsquery(FORMAT('%s', ARRAY_TO_STRING(STRING_TO_ARRAY($1, ' '), ' & '))) AS search_query,
    articles
JOIN creators ON
    articles.author = creators.username
LEFT JOIN images ON
    articles.thumbnail = images.id
WHERE
    search_query @@ articles.search_vec AND is_published = true
