
// let mut vertice: i32;
// implement graph struct

struct Node {
    id: i32,
    edges: Vec<Edge>,
}

struct Edge {
    weight: i32,
    node1: Node,
    node2: Node,
}


fn main() {
    // println!("Hello, world!");
}

// fn minDistance(dist: &[i32], spSet: &[bool]) -> usize {
    
// }

fn shortest_paths(graph: Vec<Node>, start: Node) {
    let mut visited:Vec<Node> = Vec::new();
    let mut distance:Vec<i32> = Vec::new();
    let mut unvisited = graph;
    for node in unvisited {
        distance.push(i32::MAX);
    }
    distance[start.id as usize] = 0;
    while unvisited.len() > 0 {
        let mut min = i32::MAX;
        let mut min_index = 0;
        for i in 0..unvisited.len() {
            if distance[i] < min {
                min = distance[i];
                min_index = distance[i];
            }
        }
        
    }
}

// fn create_link(node1: Node, node2: Node, weight: i32) -> Edge {
//     Edge {
//         weight: weight,
//         node1: node1,
//         node2: node2,
//     }
// }

