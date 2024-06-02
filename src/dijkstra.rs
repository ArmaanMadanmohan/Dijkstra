use crate::minheap::MinHeap;
use crate::grid::{Grid, Cell};


pub fn run(start: Cell, grid: &mut Grid, end: Cell) -> Vec<Cell> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid.dimensions.0 as usize]; grid.dimensions.1 as usize];
    let mut distance: Vec<Vec<Option<i32>>> = vec![vec![None; grid.dimensions.0 as usize]; grid.dimensions.1 as usize];
    let mut prev: Vec<Vec<Option<Cell>>> = vec![vec![None; grid.dimensions.0 as usize]; grid.dimensions.1 as usize];
    let mut path: Vec<Cell> = Vec::new();
    let mut unvisited = MinHeap::init();

    unvisited.insert(start.clone());    
    visited[start.get_coordinates().0][start.get_coordinates().1] = true;
    distance[start.get_coordinates().0][start.get_coordinates().1] = Some(0);

    while !unvisited.is_empty() {
        let current_cell = match unvisited.remove_min() {
            Some(cell) => cell,
            None => continue, // double check
        };

        let (x, y) = current_cell.get_coordinates();
        if x == end.get_coordinates().0 && y == end.get_coordinates().1 {
            break;
        }

        let neighbours = grid.adjacent(current_cell);
        for neighbour in neighbours {
            let (nx, ny) = neighbour.get_coordinates();
            if !visited[nx][ny] {
                visited[nx][ny] = true;
                distance[nx][ny] = Some(distance[x][y].unwrap() + neighbour.get_cost().unwrap());
                prev[nx][ny] = Some(current_cell.clone());
                unvisited.insert(neighbour.clone());
            }
        }
    }
    let mut current = end.clone();

    while current.get_coordinates().0 != start.get_coordinates().0 || current.get_coordinates().1 != start.get_coordinates().1 {
        path.push(current.clone());
        let (x, y) = current.get_coordinates();
        current = prev[x][y].clone().unwrap();
    }

    path.push(start.clone());
    path.reverse();

    path
}