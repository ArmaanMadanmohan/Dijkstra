
// // let mut vertice: i32;
// // implement graph struct
// #[derive(Clone)]
// struct Node {
//     id: i32,
//     edges: Vec<Edge>,
// }

// #[derive(Clone)]
// struct Edge {
//     weight: i32,
//     node1: Node,
//     node2: Node,
// }


// fn main() {
//     // println!("Hello, world!");
// }

// // fn make_graph(size: i32) -> Vec<Node> {
// //     let mut graph: Vec<Node> = Vec::new();
// //     for i in 0..size {
// //         let node = Node {
// //             id: i,
// //             edges: Vec::new(),
// //         };
// //         graph.push(node);
// //     }
// //     graph
// // }

// // fn minDistance(dist: &[i32], spSet: &[bool]) -> usize {
    
// // }

// // fn shortest_paths(graph: Vec<Node>, start: Node) -> Vec<i32> {
// //     // let mut visited: Vec<Node> = Vec::new();
// //     let mut distance: Vec<i32> = Vec::new();
// //     let mut unvisited: Vec<Node> = graph;
// //     for node in &unvisited {
// //         distance.push(i32::MAX);
// //     }
// //     distance[start.id as usize] = 0;
// //     while !unvisited.is_empty() {
// //         let mut min = i32::MAX;
// //         let mut min_index = 0;
// //         for i in 0..unvisited.len() {
// //             if distance[unvisited[i].id as usize] < min {
// //                 min = distance[unvisited[i].id as usize];
// //                 min_index = i;
// //             }
// //         }
// //         let n = unvisited.remove(min_index); 
// //         for edge in n.edges {
// //             let m = opposite(n, &edge);
// //             let d = distance[n.id as usize] + edge.weight;
// //             if d < distance[edge.node2.id as usize] {
// //                 distance[edge.node2.id as usize] = d;
// //             }
// //         } 
// //     }
// //     distance
// // }

// // fn opposite(node: &Node, edge: &Edge) -> Node {
// //      if node.id == edge.node1.id {
// //          edge.node2.clone()
// //      } else if node.id == edge.node2.id{
// //          edge.node1.clone()
// //      } else {
// //          panic!("Node is not in the edge");
// //      }
// // }

// // provide 

// fn shortest_paths(graph: Vec<Node>, start: Node) {
//     let mut visited: Vec<Node> = Vec::new();
//     let mut distance: Vec<i32> = Vec::new();

// }

fn main() {
    
}
