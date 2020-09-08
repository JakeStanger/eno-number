use crate::structs::Artist;

pub fn artist_by_id(id: i32) -> String {
    format!(
        "SELECT id, name
        FROM artist
        WHERE id = {}",
        id
    )
}

pub fn artist_acts(artist: &Artist) -> String {
    format!(
        "SELECT artist.id, artist.name
        FROM l_artist_artist
        INNER JOIN artist ON artist.id = l_artist_artist.entity1
        WHERE entity0 = {id} AND entity1 != {id}",
        id = artist.id
    )
}

pub fn act_artists(artist: &Artist) -> String {
    format!(
        "SELECT artist.id, artist.name
        FROM l_artist_artist
        INNER JOIN artist ON artist.id = l_artist_artist.entity0
        WHERE entity1 = {id} AND entity0 != {id}",
        id = artist.id
    )
}

pub fn release_artists(artist: &Artist) -> String {
    format!(
        "SELECT DISTINCT ON (artist.id, release.name)
               artist.id as artist_id, artist.name as artist_name,
               release.id as release_id, release.name as release_name
        FROM l_artist_release
                 INNER JOIN artist ON artist.id = l_artist_release.entity0
                 INNER JOIN release ON release.id = l_artist_release.entity1
        WHERE entity1 IN (
            SELECT release.id
            FROM release
            WHERE artist_credit = {id}
        ) AND artist.id != {id}
        GROUP BY artist.id, release.id;",
        id = artist.id
    )
}

pub fn search_artists(query: &String) -> String {
    // TODO: Improve ordering
    format!(
        "SELECT artist.id, name
        FROM artist
        WHERE name ILIKE '%{query}%'
           AND (EXISTS(
               SELECT 1
               FROM release
               WHERE artist_credit = artist.id
           )
            OR (
               EXISTS(
                   SELECT 1
                   FROM l_artist_artist
                   WHERE entity0 = artist.id
                      OR entity1 = artist.id
                   )
               )
            )
        ORDER BY (case when name ILIKE '{query}%' then 3 else 0 end) desc,
                 (case when name ILIKE '%{query}' then 2 else 0 end) desc,
                 (case when name ILIKE '%{query}%' then 1 else 0 end) desc,
                 name
        LIMIT 25;",
        query = query
    )
}
