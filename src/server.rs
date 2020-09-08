use rocket_contrib::database;
use rocket_contrib::databases::{postgres, redis};
use rocket_contrib::json::Json;

use crate::cache;
use crate::calculator;
use crate::database::{get_artist_by_id, search_artists};
use crate::discogs;
use crate::structs::Artist;

#[derive(Deserialize)]
struct CalculateRequestBody {
    start: i32,
    destination: i32,
}

#[derive(Serialize)]
pub struct CalculateResponseBody {
    pub path: Vec<Artist>,
    pub time: f32,
}

#[database("musicbrainz_db")]
pub struct MusicBrainzDB(postgres::Connection);

#[database("redis")]
pub struct RedisDB(redis::Connection);

#[post("/calculate", data = "<body>", format = "json")]
fn calculate(
    body: Json<CalculateRequestBody>,
    mut conn: MusicBrainzDB,
) -> Json<CalculateResponseBody> {
    let start = body.0.start;
    let destination = body.0.destination;

    // TODO: Validate inputs

    Json(calculator::calculate(start, destination, &mut *conn))
}

#[get("/search/artist/<query>")]
fn search_artist(query: String, mut conn: MusicBrainzDB) -> Json<Vec<Artist>> {
    Json(search_artists(query, &mut *conn).unwrap())
}

#[get("/art/artist/<artist_id>?<full>")]
fn get_artist_image_url(
    artist_id: i32,
    full: bool,
    mut db_conn: MusicBrainzDB,
    mut redis_conn: RedisDB,
) -> String {
    let artist = get_artist_by_id(artist_id, &mut *db_conn).unwrap();

    if let Ok(image_url) = cache::get_artist_image_url(&artist, full, &mut *redis_conn) {
        image_url
    } else {
        let image_url = discogs::get_artist_image_url(&artist, full)
            .unwrap()
            .unwrap_or("".to_string());

        cache::set_artist_image_url(&artist, full, image_url.clone(), &mut *redis_conn)
            .expect("Failed to write to redis cache");
        image_url
    }
}

pub fn start() {
    rocket::ignite()
        .attach(MusicBrainzDB::fairing())
        .attach(RedisDB::fairing())
        .mount(
            "/api",
            routes![calculate, search_artist, get_artist_image_url],
        )
        .launch();
}
