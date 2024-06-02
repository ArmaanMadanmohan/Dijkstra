use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Cell { 
    coordinates: (usize, usize), 
    element: CellElement,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CellElement {
    cell_type: CellType,
    cost: Option<i32>,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.element.cost.cmp(&other.element.cost)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    pub fn adjacent(&mut self, cell: Cell) -> Vec<Cell> {
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

    pub fn set_cell(&mut self, coords: (usize, usize), cell: Cell) {
        // let (x, y) = coordinates;
        self.data[coords.0][coords.1] = cell;
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

    pub fn in_bounds(&self, dimensions: (i32, i32)) -> bool {
        if dimensions.0 >= 0 && dimensions.1 >= 0 && dimensions.0 < self.dimensions.0 && dimensions.1 < self.dimensions.1 {
            true
        } else {
            false
        }
    }
}

impl Cell {
    pub fn new(coordinates: (usize, usize), element: CellElement) -> Self {
        Cell { coordinates, element }
    }

    pub fn get_coordinates(&self) -> (usize, usize) {
        self.coordinates
    }

    pub fn set_coordinates(&mut self, (x, y): (usize, usize)) {
        self.coordinates = (x, y);
    }

    pub fn set_cell_type(&mut self, cell_type: CellType) {
        self.element.cell_type = cell_type;
    }

    pub fn set_cell_cost(&mut self, cost: Option<i32>) {
        if let CellType::Cost = self.element.cell_type {
            self.element.cost = cost;
        } else {
            println!("Cannot set cost on a cell that is not of type Cost")
        }
    }

    pub fn get_cell_type(&self) -> CellType {
        self.element.cell_type
    }

    pub fn get_cost(&self) -> Option<i32> {
        self.element.cost
    }
}
