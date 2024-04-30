use rand::Rng;

/// Represents the different state for a cellular automaton cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellularState {
    Alive,
    Dead,
}

/// Represents a cellular automaton state at a given time.
pub struct CellularAutomaton {
    /// The width of the cellular automaton.
    pub width: usize,
    /// The height of the cellular automaton.
    pub height: usize,
    /// The cells' states of the cellular automaton, the cells' coordinates are
    /// given by their index in the vector.
    pub cells: Vec<CellularState>,
}

impl CellularAutomaton {
    /// Initializes a `CellularAutomaton` of dimensions `width`x`height`, with
    /// the cells' state assigned with a given probability.
    pub fn new(width: usize, height: usize, alive_probability: f64) -> Self {
        let cells = (0..width * height)
            .map(|_| {
                if rand::thread_rng().gen_bool(alive_probability) {
                    CellularState::Alive
                } else {
                    CellularState::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells,
        }
    }

    /// Advances the cellular automaton to the next generation based on
    /// predefined transition rules.
    ///
    /// This function applies rules to each cell, determining its state in the
    /// next generation based on the count of alive neighbors. The transition
    /// follows standard rules for Conway's Game of Life.
    ///
    /// # Remarks
    ///
    /// - Living cells with fewer than 2 or more than 3 alive neighbors die.
    /// - Dead cells with exactly 3 alive neighbors become alive.
    /// - Cells with 2 or 3 alive neighbors remain in their current state.
    pub fn transition(&mut self) {
        let mut next_generation = self.cells.clone();
        for (i, c) in self.cells.iter().enumerate() {
            let alive_neighbors =
                count_neighbors_in_state(self, i, CellularState::Alive);

            next_generation[i] = match c {
                CellularState::Alive => {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        CellularState::Dead
                    } else {
                        CellularState::Alive
                    }
                }
                CellularState::Dead => {
                    if alive_neighbors == 3 {
                        CellularState::Alive
                    } else {
                        CellularState::Dead
                    }
                }
            }
        }
        self.cells = next_generation;
    }

    /// Applies a smoothing operation to the cellular automaton, updating cell
    // states based on neighboring conditions.
    ///
    /// It iteratively processes the current generation of cells, updating each
    /// cell's state based on the count of alive neighbors. The process
    /// continues until no further changes occur.
    pub fn smooth(&mut self) {
        let mut current_generation = self.cells.clone();
        let mut has_changed = true;

        while has_changed {
            has_changed = false;
            let mut next_generation = current_generation.clone();

            for (i, c) in current_generation.iter().enumerate() {
                let alive_neighbors =
                    count_neighbors_in_state(self, i, CellularState::Alive);

                if c == &CellularState::Dead && alive_neighbors > 3 {
                    next_generation[i] = CellularState::Alive;
                    has_changed = true;
                }
            }
            current_generation = next_generation.clone();
        }
        self.cells = current_generation;
    }
}

/// Returns the flat indices of all Von Neumann neighbors for a given cell in a
/// cellular automaton, considering borders and corners.
///
/// # Arguments
///
/// `automaton`: A reference to a `CellularAutomaton` structure.
/// `i`: The flat index of the target cell in the cells vector.
///
/// # Returns
///
/// A vector containing flat indexes representing the target cell neighbors.
pub fn enumerate_neighbors(
    automaton: &CellularAutomaton,
    i: usize,
) -> Vec<usize> {
    let mut neighbors = Vec::new();
    let x = i % automaton.width;
    let y = i / automaton.width;
    // left neighbor
    if x > 0 {
        neighbors.push(i - 1);
    }
    // right neighbor
    if x < automaton.width - 1 {
        neighbors.push(i + 1);
    }
    // top neighbor
    if y > 0 {
        neighbors.push((y - 1) * automaton.width + x);
    }
    // bottom neighbor
    if y < automaton.height - 1 {
        neighbors.push((y + 1) * automaton.width + x);
    }
    // top right neighbor
    if x < automaton.width - 1 && y > 0 {
        neighbors.push((y - 1) * automaton.width + x + 1);
    }
    // top left neighbor
    if x > 0 && y > 0 {
        neighbors.push((y - 1) * automaton.width + x - 1);
    }
    // bottom right neighbor
    if x < automaton.width - 1 && y < automaton.height - 1 {
        neighbors.push((y + 1) * automaton.width + x + 1);
    }
    // bottom left neighbor
    if x > 0 && y < automaton.height - 1 {
        neighbors.push((y + 1) * automaton.width + x - 1);
    }

    neighbors
}

/// Counts the number of neighboring cells in a specified state around a given cell.
///
/// # Arguments
///
/// * `automaton` - The cellular automaton.
/// * `cell_i` - Index of the target cell.
/// * `cell_state` - The state to count in neighboring cells.
///
/// # Returns
///
/// The count of neighboring cells in the specified state.
pub fn count_neighbors_in_state(
    automaton: &CellularAutomaton,
    cell_i: usize,
    cell_state: CellularState,
) -> usize {
    enumerate_neighbors(automaton, cell_i)
        .iter()
        .filter(|&&n_i| automaton.cells[n_i] == cell_state)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_neighbors() {
        let ca1x1 = CellularAutomaton::new(1, 1, 0f64);
        let ca1x2 = CellularAutomaton::new(1, 2, 0f64);
        let ca3x3 = CellularAutomaton::new(3, 3, 0f64);

        assert_eq!(enumerate_neighbors(&ca3x3, 0).len(), 3);
        assert_eq!(enumerate_neighbors(&ca3x3, 1).len(), 5);
        assert_eq!(enumerate_neighbors(&ca3x3, 2).len(), 3);
        assert_eq!(enumerate_neighbors(&ca3x3, 3).len(), 5);
        assert_eq!(enumerate_neighbors(&ca3x3, 4).len(), 8);
        assert_eq!(enumerate_neighbors(&ca3x3, 5).len(), 5);
        assert_eq!(enumerate_neighbors(&ca3x3, 6).len(), 3);
        assert_eq!(enumerate_neighbors(&ca3x3, 7).len(), 5);
        assert_eq!(enumerate_neighbors(&ca3x3, 8).len(), 3);

        assert_eq!(enumerate_neighbors(&ca1x2, 0).len(), 1);
        assert_eq!(enumerate_neighbors(&ca1x2, 1).len(), 1);

        assert_eq!(enumerate_neighbors(&ca1x1, 0).len(), 0);
    }

    #[test]
    fn test_transition() {
        let mut ca1x1 = CellularAutomaton::new(1, 1, 1.0);
        assert_eq!(CellularState::Alive, ca1x1.cells[0]);
        for _ in 0..10 {
            ca1x1.transition();
            assert_eq!(CellularState::Dead, ca1x1.cells[0]);
        }

        // block pattern remains the same
        let mut ca2x2 = CellularAutomaton::new(2, 2, 1.0);
        for _ in 0..10 {
            ca2x2.transition();
            for i in 0..ca2x2.cells.len() {
                assert_eq!(CellularState::Alive, ca2x2.cells[i]);
            }
        }

        // blinker pattern oscillates
        let expected_blinker_even_position = vec![
            CellularState::Dead,
            CellularState::Alive,
            CellularState::Dead,
            CellularState::Dead,
            CellularState::Alive,
            CellularState::Dead,
            CellularState::Dead,
            CellularState::Alive,
            CellularState::Dead,
        ];

        let expected_blinker_odd_position = vec![
            CellularState::Dead,
            CellularState::Dead,
            CellularState::Dead,
            CellularState::Alive,
            CellularState::Alive,
            CellularState::Alive,
            CellularState::Dead,
            CellularState::Dead,
            CellularState::Dead,
        ];

        let mut ca3x3 = CellularAutomaton {
            width: 3,
            height: 3,
            cells: expected_blinker_even_position.clone(),
        };

        for i in 1..10 {
            ca3x3.transition();
            if i % 2 == 1 {
                assert_eq!(expected_blinker_odd_position.clone(), ca3x3.cells);
            } else {
                assert_eq!(expected_blinker_even_position.clone(), ca3x3.cells);
            }
        }
    }
}
