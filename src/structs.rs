#[derive(Clone, Debug, Serialize)]
pub struct Release {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub release: Option<Release>,
}

#[derive(Clone, Debug)]
pub struct ArtistNode<'a> {
    pub artist: Artist,
    pub depth: u32,
    pub parent: Option<&'a ArtistNode<'a>>,
}

impl ArtistNode<'_> {
    pub fn new<'a>(
        artist: Artist,
        parent: Option<&'a ArtistNode<'a>>,
        depth: u32,
    ) -> ArtistNode<'a> {
        ArtistNode {
            artist,
            depth,
            parent,
        }
    }

    pub fn has_visited(&self, artist: &Artist) -> bool {
        let mut parent = Some(self);
        while parent.is_some() {
            let unwrapped = parent.unwrap();
            if unwrapped.artist.id == artist.id {
                return true;
            }
            parent = unwrapped.parent;
        }

        false
    }
}
