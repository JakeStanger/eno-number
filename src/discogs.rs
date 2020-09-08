use crate::structs::Artist;
use reqwest::blocking::Client;

#[derive(Debug, Deserialize, Clone)]
struct DiscogsPaginationUrls {
    last: Option<String>,
    next: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct DiscogsPagination {
    page: u32,
    pages: u32,
    per_page: u32,
    items: u32,
    urls: DiscogsPaginationUrls,
}

#[derive(Debug, Deserialize, Clone)]
struct DiscogsArtist {
    id: i32,
    #[serde(rename = "type")]
    type_: String,
    master_id: Option<i32>,
    master_url: Option<String>,
    title: String,
    thumb: String,
    cover_image: String,
    resource_url: String,
}

#[derive(Debug, Deserialize)]
struct DiscogsResults {
    pagination: DiscogsPagination,
    results: Vec<DiscogsArtist>,
}

pub fn get_artist_image_url(artist: &Artist, full: bool) -> Result<Option<String>, reqwest::Error> {
    let request_url = format!(
        "https://api.discogs.com/database/search?type=artist&q={query}&key={key}&secret={secret}",
        query = artist.name,
        key = dotenv!("DISCOGS_API_KEY"),
        secret = dotenv!("DISCOGS_API_SECRET")
    );

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let client = Client::builder().user_agent(APP_USER_AGENT).build()?;

    let response = client.get(&request_url).send()?;

    let results = response.json::<DiscogsResults>().unwrap().results;

    // fetch the first exact result, otherwise the first result
    if !results.is_empty() {
        let result_artist: DiscogsArtist = results
            .iter()
            .find(|&result| result.title == artist.name)
            .unwrap_or(results.first().unwrap())
            .clone();

        // some images have cover images but not thumbs
        Ok(Some(if full || result_artist.thumb.is_empty() {
            result_artist.cover_image
        } else {
            result_artist.thumb
        }))
    } else {
        Ok(None)
    }
}
