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
        let cells = (0..width * height)
            .map(|_| {
                if rand::thread_rng().gen_bool(alive_probability) {
                    CellularState::Dead
                } else {
                    CellularState::Alive
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn transition(&mut self) {
        let mut next_generation = self.cells.clone();
        for (i, c) in self.cells.iter().enumerate() {
            let alive_neighbours =
                count_neighbours_in_state(self, i, CellularState::Alive);

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
                let alive_neighbours =
                    count_neighbours_in_state(self, i, CellularState::Alive);

                if c == &CellularState::Dead && alive_neighbours > 3u8 {
                    next_generation[i] = CellularState::Alive;
                    has_changed = true;
                }
            }
            current_generation = next_generation.clone();
        }
        self.cells = current_generation;
    }
}

pub fn enumerate_neighbours(
    automaton: &CellularAutomaton,
    i: usize,
) -> Vec<usize> {
    let mut neighbours = Vec::new();
    let x = i % automaton.width;
    let y = i / automaton.width;
    // left neighbour
    if x > 0 {
        neighbours.push(i - 1);
    }
    // right neighbour
    if x < automaton.width - 1 {
        neighbours.push(i + 1);
    }
    // top neighbour
    if y > 0 {
        neighbours.push((y - 1) * automaton.width + x);
    }
    // bottom neighbour
    if y < automaton.height - 1 {
        neighbours.push((y + 1) * automaton.width + x);
    }
    // top right neighbour
    if x < automaton.width - 1 && y > 0 {
        neighbours.push((y - 1) * automaton.width + x + 1);
    }
    // top left neighbour
    if x > 0 && y > 0 {
        neighbours.push((y - 1) * automaton.width + x - 1);
    }
    // bottom right neighbour
    if x < automaton.width - 1 && y < automaton.height - 1 {
        neighbours.push((y + 1) * automaton.width + x + 1);
    }
    // bottom left neighbour
    if x > 0 && y < automaton.height - 1 {
        neighbours.push((y + 1) * automaton.width + x - 1);
    }

    neighbours
}

pub fn count_neighbours_in_state(
    automaton: &CellularAutomaton,
    cell_i: usize,
    cell_state: CellularState,
) -> u8 {
    let neighbours = enumerate_neighbours(automaton, cell_i);
    let mut neighbours_in_state = 0u8;
    for n_i in neighbours {
        if automaton.cells[n_i] == cell_state {
            neighbours_in_state += 1u8;
        }
    }
    neighbours_in_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_neighbours() {
        let ca1x1 = CellularAutomaton::new(1, 1, 0f64);
        let ca1x2 = CellularAutomaton::new(1, 2, 0f64);
        let ca3x3 = CellularAutomaton::new(3, 3, 0f64);

        assert_eq!(enumerate_neighbours(&ca3x3, 0).len(), 3);
        assert_eq!(enumerate_neighbours(&ca3x3, 1).len(), 5);
        assert_eq!(enumerate_neighbours(&ca3x3, 2).len(), 3);
        assert_eq!(enumerate_neighbours(&ca3x3, 3).len(), 5);
        assert_eq!(enumerate_neighbours(&ca3x3, 4).len(), 8);
        assert_eq!(enumerate_neighbours(&ca3x3, 5).len(), 5);
        assert_eq!(enumerate_neighbours(&ca3x3, 6).len(), 3);
        assert_eq!(enumerate_neighbours(&ca3x3, 7).len(), 5);
        assert_eq!(enumerate_neighbours(&ca3x3, 8).len(), 3);

        assert_eq!(enumerate_neighbours(&ca1x2, 0).len(), 1);
        assert_eq!(enumerate_neighbours(&ca1x2, 1).len(), 1);

        assert_eq!(enumerate_neighbours(&ca1x1, 0).len(), 0);
    }
}
