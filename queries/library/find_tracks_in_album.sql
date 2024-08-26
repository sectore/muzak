SELECT * FROM TRACKS
WHERE album_id = $1
ORDER BY track_number ASC;
