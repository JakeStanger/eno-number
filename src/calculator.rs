use std::time::Instant;

use crate::database::{get_artist_by_id, get_associated_artists};
use crate::path_vec::PathVec;
use crate::server::CalculateResponseBody;
use crate::structs::{Artist, ArtistNode};
use rocket_contrib::databases::postgres;
use std::collections::HashMap;

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

fn has_not_explored(artist: &Artist, explored: &mut HashMap<i32, u32>, depth: u32) -> bool {
    !explored.contains_key(&artist.id) || explored.get(&artist.id).unwrap() > &depth
}

/// Gets a list of paths that join two artists.
/// Recursive main calculation method.
fn get_associations(
    artist_node: &ArtistNode,
    destination: &Artist,
    paths: &mut PathVec,
    explored: &mut HashMap<i32, u32>,
    database: &mut postgres::Connection,
) {
    let associations = get_associated_artists(&artist_node.artist, database);

    for association in associations
        .into_iter()
        .filter(|artist| {
            !artist_node.has_visited(artist)
                && has_not_explored(artist, explored, artist_node.depth)
        })
        .collect::<Vec<_>>()
    {
        let node = ArtistNode::new(association, Some(artist_node), artist_node.depth + 1);

        for _i in 0..node.depth {
            print!("\t");
        }
        println!("[{}]\t {}", node.depth, node.artist.name);

        // route found
        if node.artist.id == destination.id {
            let mut route = Vec::new();

            let mut parent = Some(&node);
            while parent.is_some() {
                let unwrapped = parent.unwrap();
                route.push(unwrapped.clone().artist);
                parent = unwrapped.parent;
            }
            route.reverse();
            paths.add_path(route.clone()); // TODO: Remove clone (after log cleaned)

            println!("===ROUTE FOUND===");
            println!("Length: {}", route.len());
            println!("Shortest: {}", paths.shortest_distance);
            println!("{}", format_path(route));
            println!("------\n");
        } else if node.depth < paths.shortest_distance as u32 {
            get_associations(&node, destination, paths, explored, database);
        }
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

    let start_node: ArtistNode = ArtistNode::new(start.clone(), None, 1);

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
