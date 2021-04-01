#[derive(Clone, Debug)]
pub struct RowRecord<Cell> {
    cells: Vec<Cell>,
}

impl<Cell> RowRecord<Cell> {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self { cells }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut()
    }

    pub fn cell(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    pub fn cell_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }

    pub fn into_cells(self) -> Vec<Cell> {
        self.cells
    }
}
