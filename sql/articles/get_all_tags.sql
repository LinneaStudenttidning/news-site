SELECT ARRAY (SELECT tag FROM 
    (SELECT UNNEST(tags) as tag FROM articles WHERE is_published = true) 
GROUP BY tag ORDER BY COUNT(*) DESC LIMIT $1);
