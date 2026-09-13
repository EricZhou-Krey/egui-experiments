#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub cells: Vec<bool>,
    pub cols: usize,
    pub rows: usize,
}

impl Board {
    pub fn new(cols: usize, rows: usize, initial_density: f32) -> Self {
        let total_cells: usize = cols * rows;
        let mut cells: Vec<bool> = Vec::with_capacity(total_cells);

        for _ in 0..total_cells {
            let is_alive: bool = rand::random::<f32>() < initial_density;
            cells.push(is_alive);
        }

        Self { cells, cols, rows }
    }

    pub fn step(&mut self) {
        let mut next_generation: Vec<bool> = vec![false; self.cells.len()];

        for y in 0..self.rows {
            for x in 0..self.cols {
                let current_index: usize = y * self.cols + x;
                let alive_neighbors: u8 = self.count_neighbors(x, y);
                let currently_alive: bool = self.cells[current_index];

                next_generation[current_index] = match (currently_alive, alive_neighbors) {
                    (true, 2) | (true, 3) => true, // Survival
                    (false, 3) => true,            // Reproduction
                    _ => false,                    // Under/Overpopulation
                };
            }
        }

        self.cells = next_generation;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        let offsets: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for (dx, dy) in offsets {
            // Toroidal wrapping using modulo
            let nx: usize = (x as isize + dx).rem_euclid(self.cols as isize) as usize;
            let ny: usize = (y as isize + dy).rem_euclid(self.rows as isize) as usize;

            if self.cells[ny * self.cols + nx] {
                count += 1;
            }
        }
        count
    }
}
