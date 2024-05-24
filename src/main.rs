pub mod minheap;
use crate::{
    minheap::MinHeap,
    minheap::Vertex,
};
use std::collections::HashMap;


fn shortest_path<'a>(
    start: Vertex<'a>,
    matrix: &Vec<Vec<Option<usize>>>,
    vertices: &'a Vec<Vertex<'a>>,
) -> HashMap<&'a str, Option<usize>> {
    let mut unvisited = MinHeap::init();
    let mut visited = vec![false; vertices.len()];
    let mut distances: HashMap<&'a str, Option<usize>> = HashMap::new();

    for vertex in vertices {
        distances.insert(vertex.name, None);
    }
    distances.insert(start.name, Some(0));
    unvisited.insert(Vertex {
        name: start.name,
        distance: 0,
    });

    while let Some(Vertex { name, distance }) = unvisited.remove_min() {
        let index = match vertices.iter().position(|v| v.name == name) {
            Some(index) => index,
            None => continue,
        };

        if visited[index] {
            continue;
        }

        visited[index] = true;

        for (neighbor_idx, &weight) in matrix[index].iter().enumerate() {
            if weight.is_none() {
                continue;
            }

            let weight = weight.unwrap();
            if !visited[neighbor_idx] {
                let new_distance = distance + weight;
                let neighbor_name = vertices[neighbor_idx].name;
                if distances[neighbor_name].is_none() || new_distance < distances[neighbor_name].unwrap() {
                    distances.insert(neighbor_name, Some(new_distance));
                    unvisited.insert(Vertex {
                        name: neighbor_name,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}

fn main() {
    let vertices = vec![
        Vertex { name: "s", distance: 0 },
        Vertex { name: "t", distance: 0 },
        Vertex { name: "x", distance: 0 },
        Vertex { name: "y", distance: 0 },
    ];

    let matrix = vec![
        vec![None, Some(2), Some(3),None],
        vec![Some(2), None, None, Some(2)],
        vec![Some(3), None, None, Some(6)],
        vec![None, Some(2), Some(6), None],
    ];

    let start = vertices[0];
    let distances = shortest_path(start, &matrix, &vertices);

    for (v, d) in &distances {
        println!("name: {}, distance: {:?}", v, d);
    }
}

//neighbour_idx or column? current idx or row?