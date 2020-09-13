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
    pub depth: u8,
    pub parent: Option<&'a ArtistNode<'a>>,
}

impl ArtistNode<'_> {
    /// Checks if the path to get to this node
    /// already visits the given artist.
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
