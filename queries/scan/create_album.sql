INSERT INTO album (title, title_sortable, artist_id, image)
    VALUES ($1, $2, $3, $4)
    ON CONFLICT (title, artist_id) DO NOTHING
    RETURNING id;
