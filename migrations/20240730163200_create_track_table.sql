CREATE TABLE IF NOT EXISTS track (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    title_sortable TEXT NOT NULL,
    album_id INTEGER NOT NULL,
    artist_id INTEGER NOT NULL,
    track_number INTEGER NOT NULL,
    disc_number INTEGER NOT NULL,
    duration INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    genres TEXT,
    tags TEXT,
    location TEXT NOT NULL,
    FOREIGN KEY (album_id) REFERENCES album (id),
    FOREIGN KEY (artist_id) REFERENCES artist (id)
)
