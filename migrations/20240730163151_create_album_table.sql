CREATE TABLE IF NOT EXISTS album (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    title_sortable TEXT NOT NULL,
    artist_id INTEGER NOT NULL,
    release_date DATE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    image BLOB,
    image_mime TEXT,
    tags TEXT,
    FOREIGN KEY (artist_id) REFERENCES artist (id)
)
