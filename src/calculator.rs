use std::collections::HashMap;
use std::time::Instant;

use rocket_contrib::databases::postgres;

use crate::database::{get_artist_by_id, get_associated_artists};
use crate::path_vec::PathVec;
use crate::server::CalculateResponseBody;
use crate::structs::{Artist, ArtistNode};

/// Converts an vector of Artists
/// into a string, joined by `-->`.
fn format_path(path: Vec<Artist>) -> String {
    format!(
        "[{}] {}",
        path.len() - 1,
        path.into_iter()
            .map(|artist| format!(
                "{}{}",
                artist.name,
                if artist.release.is_some() {
                    format!(" ({})", artist.release.unwrap().name)
                } else {
                    "".to_string()
                }
            ))
            .collect::<Vec<String>>()
            .join(" --> ")
    )
}

/// Prints a node as part of a tree.
fn print_node(node: &ArtistNode) {
    for _i in 0..node.depth {
        print!("\t");
    }
    println!(
        "[{}]\t {}{}",
        node.depth,
        node.artist.name,
        if (&node).artist.release.is_some() {
            format!(" ({})", &(node).artist.release.as_ref().unwrap().name)
        } else {
            "".to_string()
        }
    );
}

/// Checks whether an artist has already been explored.
///
/// Returns false if the artist has been explored,
/// but at a higher depth than the current.
fn has_explored(artist: &Artist, explored: &mut HashMap<i32, u8>, current_depth: u8) -> bool {
    let explored_depth = explored.get(&artist.id);
    explored_depth.is_some() && &current_depth >= explored_depth.unwrap()
}

fn add_node(node: &ArtistNode, paths: &mut PathVec) {
    let mut route = Vec::new();

    let mut parent = Some(node);
    while parent.is_some() {
        let unwrapped = parent.unwrap();
        route.push(unwrapped.clone().artist);
        parent = unwrapped.parent;
    }
    route.reverse();
    paths.add_path(route);
}

/// Gets a list of paths that join two artists.
/// Recursive main calculation method.
fn get_associations(
    artist_node: &ArtistNode,
    destination: &Artist,
    paths: &mut PathVec,
    explored: &mut HashMap<i32, u8>,
    database: &mut postgres::Connection,
) {
    let associations = get_associated_artists(&artist_node.artist, database);

    for association in associations {
        // ignore previously visited artists
        if artist_node.has_visited(&association)
            || has_explored(&association, explored, artist_node.depth)
        {
            continue;
        }

        let node = ArtistNode {
            artist: association,
            parent: Some(artist_node),
            depth: artist_node.depth + 1,
        };

        print_node(&node);

        // route found
        if node.artist.id == destination.id {
            add_node(&node, paths);
        } else if node.depth < paths.shortest_distance as u8 {
            get_associations(&node, destination, paths, explored, database);
        }

        explored.insert(node.artist.id, node.depth);
    }

    explored.insert(artist_node.artist.id, artist_node.depth);
}

/// Gets the shortest path between two artists.
pub fn calculate(
    start: i32,
    destination: i32,
    database: &mut postgres::Connection,
) -> CalculateResponseBody {
    let start_time = Instant::now();

    let start = get_artist_by_id(start, database).unwrap();
    let destination = get_artist_by_id(destination, database).unwrap();

    println!("Start:\t\t\t[{}]\t{}", start.clone().id, start.name);
    println!(
        "Destination:\t[{}]\t{}",
        destination.clone().id,
        destination.name
    );

    println!("[1] {}", start.name);

    let start_node: ArtistNode = ArtistNode {
        artist: start.clone(),
        parent: None,
        depth: 1,
    };

    let mut paths = PathVec::new();
    let mut explored = HashMap::new();
    get_associations(
        &start_node,
        &destination,
        &mut paths,
        &mut explored,
        database,
    );

    let elapsed_time = start_time.elapsed().as_secs_f32();

    println!("===RESULTS===");
    println!(
        "Found {} paths in {:.2} seconds",
        paths.paths.len(),
        elapsed_time
    );

    println!("Paths:");
    for path in paths.paths.clone() {
        println!("{}", format_path(path));
    }

    if let Some(shortest_path) = paths.shortest_path() {
        println!("\nShortest:");
        let formatted = format_path(shortest_path.clone());
        println!("{}", formatted);

        return CalculateResponseBody {
            path: shortest_path,
            time: elapsed_time,
        };
    }

    CalculateResponseBody {
        path: Vec::new(),
        time: elapsed_time,
    }
}
