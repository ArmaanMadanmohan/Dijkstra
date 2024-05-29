// pub mod minheap;
// use crate::minheap::Vertex; // find a way to not repeat these among main etc..

#[derive(Debug, Clone)]
pub enum CellType { //vertex struct?
    Start,
    End,
    Wall,
    Cost, 
    Blank,
}

pub struct Grid {
    pub dimensions: (i32, i32),
    pub data: Vec<Vec<Cell>>, 
}

#[derive(Debug, Clone)]
pub struct Cell { 
    // vertex: Vertex,
    coordinates: (usize, usize), 
    element: CellElement,
}

#[derive(Debug, Clone)]
pub struct CellElement {
    cell_type: CellType,
    cost: Option<i32>,
}

//compare cells function? implement if required. instead of cloning might just create duplicate

impl Grid {
    /**
     * Initialises new grid with given dimensions
     */
    pub fn new(size: (i32, i32)) -> Grid {
        let mut col = Vec::new();
        for x in 0..size.0 {
            let mut row = Vec::new();   
            for y in 0..size.1 {
                let cell_element = CellElement { cell_type: CellType::Blank, cost: None };
                row.push(Cell {
                    coordinates: (x as usize, y as usize),
                    element: cell_element,
                });
            }
            col.push(row); 
        }
        Grid { dimensions: size, data: col }
    }

    /**
     * Returns an array of adjacent/neighbouring cells to the given cell.
     */
    pub fn adjacent(&self, cell: Cell) -> Vec<Cell> {
        let mut neighbours: Vec<Cell> = Vec::new();

        if cell.coordinates.0 > 0 { 
            neighbours.push(self.get_cell((cell.coordinates.0 - 1, cell.coordinates.1)));
        }
        if cell.coordinates.0 < (self.dimensions.0 - 1) as usize {
            neighbours.push(self.get_cell((cell.coordinates.0 + 1, cell.coordinates.1)));
        }
        if cell.coordinates.1 > 0 {
            neighbours.push(self.get_cell((cell.coordinates.0, cell.coordinates.1 - 1)));
        }
        if cell.coordinates.1 < (self.dimensions.1 - 1) as usize {
            neighbours.push(self.get_cell((cell.coordinates.0, cell.coordinates.1 + 1)));
        }

        neighbours
    }

    /**
     * Returns cell at given co-ordinates
     */
    pub fn get_cell(&self, coords: (usize, usize)) -> Cell {
        self.data[coords.0][coords.1].clone()
    }

    /**
     * Returns a flattened array of all cells in a grid
     */
    pub fn get_all_cells(&self) -> Vec<Cell> {
        let mut returned_cells = Vec::new();
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                returned_cells.push(self.get_cell((x as usize, y as usize)));
            }
        }
        returned_cells
    }
}

impl Cell {
    /**
     * Adjusts cell type
     */
    pub fn set_cell_type(&mut self, cell_type: CellType) {
        self.element.cell_type = cell_type;
    }

    /**
     * Adjusts cell cost (to None or a provided number)
     */
    pub fn set_cell_cost(&mut self, cost: Option<i32>) { //change to Result, use Ok(()) and Err() for error handling?
        if let CellType::Cost = self.element.cell_type {
                self.element.cost = cost;
        }
        else {
            println!("Cannot set cost on a cell that is not of type Cost")
        }     
    }
}

fn main() {
    let size: (i32, i32) = (5,5);
    let grid = Grid::new(size);
    let mut cell = grid.get_cell((2, 3));
    println!("cell: {:?}", cell);
    // cell.set_cell_type(CellType::Cost);
    cell.set_cell_cost(Some(5));
    println!("cell: {:?}", cell);
    // println!("coords: {:?}", cell.coordinates);
    // println!("type: {:?}\n", cell.element.cell_type);

    // let neighbours = grid.adjacent(cell);
    // for i in neighbours {
    //     println!("adjacent cell coords {:?}", i.coordinates);
    //     println!("adjacent cell type {:?}\n", i.element.cell_type);
    // }
}

// pub struct Dimensions {
//     x: i32,
//     y: i32,
// }

// impl Dimensions {
//     pub fn new(length: i32, height: i32) -> Dimensions {
//         Dimensions { length, height }
//     }
// }