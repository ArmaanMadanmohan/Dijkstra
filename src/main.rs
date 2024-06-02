pub mod minheap;
pub mod grid;
pub mod dijkstra;

use crate::grid::{Grid, CellType};
use crate::dijkstra::run;

fn main() {
    let mut grid = Grid::new((5, 5));

    for x in 0..grid.dimensions.0 {
        for y in 0..grid.dimensions.1 {
            let mut cell = grid.get_cell((x as usize, y as usize));
            cell.set_cell_type(CellType::Cost);
            cell.set_cell_cost(Some((x * 30) + (y * 20))); // Set cost based on coordinates for example
            grid.set_cell((x as usize, y as usize), cell);
        }
    }

    let start_cell = grid.get_cell((0, 0));
    let end_cell = grid.get_cell((4, 4));

    let path = run(start_cell, &mut grid, end_cell);
    println!("Path: {:?}", path);
}