use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub enum CellularState {
    Alive,
    Dead,
}

pub struct CellularAutomaton {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<CellularState>,
}

impl CellularAutomaton {
    pub fn new(width: usize, height: usize, alive_probability: f64) -> Self {
        let mut cells: Vec<CellularState> = Vec::<CellularState>::new();
        let mut rng = rand::thread_rng();
        for i in 0..(width * height) {
            if rng.gen_bool(alive_probability) {
                cells.push(CellularState::Dead);
            } else {
                cells.push(CellularState::Alive);
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    // collect von neumann neighbours of a position
    pub fn collect_neighbours_from(
        &self,
        x: usize,
        y: usize,
    ) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        // left neighbour
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        // right neighbour
        if x < self.width - 1 {
            neighbours.push((x + 1, y));
        }
        // top neighbour
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        // bottom neighbour
        if y < self.height - 1 {
            neighbours.push((x, y + 1));
        }
        // top right neighbour
        if x < self.width - 1 && y > 0 {
            neighbours.push((x + 1, y - 1));
        }
        // top left neighbour
        if x > 0 && y > 0 {
            neighbours.push((x - 1, y - 1));
        }
        // bottom right neighbour
        if x < self.width - 1 && y < self.height - 1 {
            neighbours.push((x + 1, y + 1));
        }
        // bottom left neighbour
        if x > 0 && y < self.height - 1 {
            neighbours.push((x - 1, y + 1));
        }

        neighbours
    }

    pub fn transition(&mut self) {
        let mut next_generation = self.cells.clone();
        for (i, c) in self.cells.iter().enumerate() {
            let c_x = i % self.width;
            let c_y = i / self.width;
            let c_neighbours = self.collect_neighbours_from(c_x, c_y);
            let alive_neighbours = {
                let mut a = 0;
                for c in c_neighbours {
                    match self.cells[c.0 + c.1 * self.width] {
                        CellularState::Alive => a = a + 1,
                        CellularState::Dead => {}
                    }
                }
                a
            };

            next_generation[i] = match c {
                CellularState::Alive => {
                    if alive_neighbours < 2 || alive_neighbours > 3 {
                        CellularState::Dead
                    } else {
                        CellularState::Alive
                    }
                }
                CellularState::Dead => {
                    if alive_neighbours == 3 {
                        CellularState::Alive
                    } else {
                        CellularState::Dead
                    }
                }
            }
        }
        self.cells = next_generation;
    }

    pub fn smooth(&mut self) {
        let mut current_generation = self.cells.clone();
        let mut has_changed = true;

        while has_changed {
            has_changed = false;
            let mut next_generation = current_generation.clone();

            for (i, c) in current_generation.iter().enumerate() {
                let c_x = i % self.width;
                let c_y = i / self.width;
                let c_neighbours = self.collect_neighbours_from(c_x, c_y);
                let alive_neighbours = {
                    let mut a = 0;
                    for c in c_neighbours {
                        match current_generation[c.0 + c.1 * self.width] {
                            CellularState::Alive => a = a + 1,
                            CellularState::Dead => {}
                        }
                    }
                    a
                };

                if c == &CellularState::Dead && alive_neighbours > 3 {
                    next_generation[i] = CellularState::Alive;
                    has_changed = true;
                }
            }
            current_generation = next_generation.clone();
        }
        self.cells = current_generation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_neighbours() {
        let ca1x1 = CellularAutomaton::new(1, 1, 0f64);
        let ca1x2 = CellularAutomaton::new(1, 2, 0f64);
        let ca3x3 = CellularAutomaton::new(3, 3, 0f64);

        let pos00_neighbours = ca3x3.collect_neighbours_from(0, 0);
        let pos01_neighbours = ca3x3.collect_neighbours_from(0, 1);
        let pos02_neighbours = ca3x3.collect_neighbours_from(0, 2);
        let pos10_neighbours = ca3x3.collect_neighbours_from(1, 0);
        let pos11_neighbours = ca3x3.collect_neighbours_from(1, 1);
        let pos12_neighbours = ca3x3.collect_neighbours_from(1, 2);
        let pos20_neighbours = ca3x3.collect_neighbours_from(2, 0);
        let pos21_neighbours = ca3x3.collect_neighbours_from(2, 1);
        let pos22_neighbours = ca3x3.collect_neighbours_from(2, 2);

        assert_eq!(pos00_neighbours.len(), 3);
        assert_eq!(pos01_neighbours.len(), 5);
        assert_eq!(pos02_neighbours.len(), 3);
        assert_eq!(pos10_neighbours.len(), 5);
        assert_eq!(pos11_neighbours.len(), 8);
        assert_eq!(pos12_neighbours.len(), 5);
        assert_eq!(pos20_neighbours.len(), 3);
        assert_eq!(pos21_neighbours.len(), 5);
        assert_eq!(pos22_neighbours.len(), 3);

        let pos00_neighbours = ca1x2.collect_neighbours_from(0, 0);
        let pos01_neighbours = ca1x2.collect_neighbours_from(0, 1);
        assert_eq!(pos00_neighbours.len(), 1);
        assert_eq!(pos01_neighbours.len(), 1);

        let pos00_neighbours = ca1x1.collect_neighbours_from(0, 0);
        assert_eq!(pos00_neighbours.len(), 0);
    }
}
