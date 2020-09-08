use crate::structs::Artist;
use rocket_contrib::databases::redis::{self, Commands};

fn get_artist_image_key(artist: &Artist, full: bool) -> String {
    format!(
        "image-{}-{}",
        artist.id,
        if full { "full" } else { "thumb" }
    )
}

pub fn get_artist_image_url(
    artist: &Artist,
    full: bool,
    conn: &mut redis::Connection,
) -> Result<String, redis::RedisError> {
    let val: String = conn.get(get_artist_image_key(artist, full))?;
    Ok(val)
}

pub fn set_artist_image_url(
    artist: &Artist,
    full: bool,
    url: String,
    conn: &mut redis::Connection,
) -> redis::RedisResult<()> {
    conn.set(get_artist_image_key(artist, full), url)
}
