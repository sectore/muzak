use std::path::Path;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use tracing::debug;

use super::types::Album;

pub async fn create_pool(path: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
    debug!("Creating database pool at {:?}", path.as_ref());
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlbumSortMethod {
    TitleAsc,
    TitleDesc,
}

pub async fn list_albums(
    pool: &SqlitePool,
    sort_method: AlbumSortMethod,
) -> Result<Vec<Album>, sqlx::Error> {
    let query = match sort_method {
        AlbumSortMethod::TitleAsc => {
            include_str!("../../queries/library/find_albums_title_asc.sql")
        }
        AlbumSortMethod::TitleDesc => {
            include_str!("../../queries/library/find_albums_title_desc.sql")
        }
    };

    let albums = sqlx::query_as::<_, Album>(query).fetch_all(pool).await?;

    Ok(albums)
}

pub async fn list_tracks_in_album(
    pool: &SqlitePool,
    album_id: i64,
) -> Result<Vec<Album>, sqlx::Error> {
    let query = include_str!("../../queries/library/find_tracks_in_album.sql");

    let albums = sqlx::query_as::<_, Album>(query)
        .bind(album_id)
        .fetch_all(pool)
        .await?;

    Ok(albums)
}
