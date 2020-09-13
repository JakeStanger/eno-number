use postgres::rows::Row;
use rocket_contrib::databases::postgres;

use crate::database::queries::{artist_by_id, associated_acts, release_artists};
use crate::settings::{INCLUDE_RECORDINGS, INCLUDE_RELEASES};
use crate::structs::{Artist, Release};

mod queries;

type ArtistQueryResult = Result<Vec<Artist>, postgres::Error>;

fn get_artist_from_row(row: Row) -> Artist {
    let release: Option<Release> = if row.len() == 4 {
        Some(Release {
            id: row.get(2),
            name: row.get(3),
        })
    } else {
        None
    };

    Artist {
        id: row.get(0),
        name: row.get(1),
        release,
    }
}

fn get_associated_acts(artist: &Artist, client: &mut postgres::Connection) -> ArtistQueryResult {
    let rows = client.query(associated_acts(artist).as_str(), &[])?;

    Ok(rows.iter().map(|row| get_artist_from_row(row)).collect())
}

fn get_release_artists(artist: &Artist, client: &mut postgres::Connection) -> ArtistQueryResult {
    let rows = client.query(release_artists(artist).as_str(), &[])?;

    Ok(rows.iter().map(|row| get_artist_from_row(row)).collect())
}

pub fn get_associated_artists(artist: &Artist, client: &mut postgres::Connection) -> Vec<Artist> {
    let mut associated: Vec<Artist> = Vec::new();
    associated.append(&mut get_associated_acts(artist, client).unwrap());

    if INCLUDE_RELEASES {
        associated.append(&mut get_release_artists(artist, client).unwrap());
    }

    if INCLUDE_RECORDINGS {
        // TODO: Fetch associated artists through recordings
    }

    associated
}

pub fn search_artists(query: String, client: &mut postgres::Connection) -> ArtistQueryResult {
    // TODO: Cache the results in redis once the algorithm is improved
    let rows = client.query(queries::search_artists(&query).as_str(), &[])?;

    Ok(rows.iter().map(|row| get_artist_from_row(row)).collect())
}

pub fn get_artist_by_id(
    id: i32,
    client: &mut postgres::Connection,
) -> Result<Artist, postgres::Error> {
    let row = client.query(artist_by_id(id).as_str(), &[])?;

    Ok(get_artist_from_row(row.get(0)))
}
