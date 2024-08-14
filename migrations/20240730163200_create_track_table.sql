CREATE TABLE IF NOT EXISTS track (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    title_sortable TEXT NOT NULL,
    album_id INTEGER,
    track_number INTEGER,
    disc_number INTEGER,
    duration INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    genres TEXT,
    tags TEXT,
    location TEXT NOT NULL,
    FOREIGN KEY (album_id) REFERENCES album (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS track_title_album_id_idx ON track (title, album_id);
