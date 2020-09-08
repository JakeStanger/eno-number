use crate::settings::MAX_DISTANCE;
use crate::structs::Artist;

pub struct PathVec {
    pub paths: Vec<Vec<Artist>>,
    pub shortest_distance: usize,
}

impl PathVec {
    pub fn new() -> PathVec {
        PathVec {
            paths: Vec::new(),
            shortest_distance: MAX_DISTANCE as usize,
        }
    }

    pub fn add_path(&mut self, path: Vec<Artist>) {
        self.paths.push(path.clone());

        // distance from the start point
        if path.len() - 1 < self.shortest_distance {
            self.shortest_distance = path.len() - 1;
        }
    }

    pub fn shortest_path(&self) -> Option<Vec<Artist>> {
        if self.paths.is_empty() {
            None
        } else {
            Some(
                self.paths
                    .iter()
                    .fold(self.paths[0].clone(), |shortest, artist| {
                        if artist.len() < shortest.len() {
                            artist.clone()
                        } else {
                            shortest
                        }
                    }),
            )
        }
    }
}
