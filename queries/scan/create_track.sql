INSERT INTO track (title, title_sortable, album_id, track_number, disc_number, duration, location, genres)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ON CONFLICT (title, album_id) DO UPDATE SET
        title = EXCLUDED.title,
        title_sortable = EXCLUDED.title_sortable,
        album_id = EXCLUDED.album_id,
        track_number = EXCLUDED.track_number,
        disc_number = EXCLUDED.disc_number,
        duration = EXCLUDED.duration,
        location = EXCLUDED.location,
        genres = EXCLUDED.genres
    RETURNING id;
