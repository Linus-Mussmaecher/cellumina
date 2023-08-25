use crate::CellGrid;

/// An environment rule uses the neighborhood (up to a certain range as specified) of a cell and applies a function to it.
/// The result of this function is the next value of that cell.
/// Applying this to each cell yields the entire transformation.
///
/// Note that each application of the ```cell_transform``` function will read from the entire untransformed array.
/// Also, the environment will wrap around the grid edges.
/// ```
/// use cellumina::rule::Rule;
/// let rule = cellumina::rule::EnvironmentRule {
///     row_range: 1,
///     col_range: 1,
///     boundaries: (cellumina::rule::BoundaryBehaviour::Periodic, cellumina::rule::BoundaryBehaviour::Periodic),
///     cell_transform: |env: &cellumina::CellGrid| match env
///     // Iterate over neighbors.
///         .iter()
///         .enumerate()
///         .map(|val| match val {
///             // The cell we are transforming does not get counted.
///             (4, 'X') => 0,
///             // Any cell containing an 'X' counts for 1 (alive).
///             (_, 'X') => 1,
///             // Any cell containing any other entry (only ' ' in our initial configuration) counts as 0 (dead).
///             _ => 0,
///         })
///         // Sum over these 9 values...
///         .sum()
///         // ... and map the sum to the new enty of our cell:
///     {
///         // 2 neighbors: The cell keeps its state.
///         2 => env[1][1],
///         // 3 neighbors: The cell gets born.
///         3 => 'X',
///         // 0, 1 or more than 3 neighbors: The cell dies.
///         _ => ' ',
///     },
/// };
/// let mut grid = grid::grid![[' ', ' ', 'X', ' ', ' '][' ', ' ', 'X',' ', ' '][' ', ' ', ' ', ' ', ' '][' ', ' ', 'X', ' ', ' '][' ', ' ', 'X', ' ', ' ']];
/// rule.transform(&mut grid);
/// assert_eq!(
///     grid,
///     grid::grid![[' ', 'X', 'X', 'X', ' '][' ', ' ', ' ',' ', ' '][' ', ' ', ' ', ' ', ' '][' ', ' ', ' ', ' ', ' '][' ', 'X', 'X', 'X', ' ']]
/// );
/// rule.transform(&mut grid);
/// rule.transform(&mut grid);
/// rule.transform(&mut grid);
/// assert_eq!(
///     grid,
///     grid::grid![[' ', 'X', ' ', 'X', ' '][' ', ' ', 'X',' ', ' '][' ', ' ', ' ', ' ', ' '][' ', ' ', 'X', ' ', ' '][' ', 'X', ' ', 'X', ' ']]
/// );
/// ```
#[derive(Clone, Copy)]
pub struct EnvironmentRule {
    /// The vertical range of an environment, extending in both direction from the cell to be transformed.
    ///
    /// Your ```cell_transform``` function will receive a grid of height ```2 * row_range + 1```, centered on the cell that will be replaced by the output.
    pub row_range: usize,
    /// The horizontal range of an environment, extending in both direction from the cell to be transformed.
    ///
    /// Your ```cell_transform``` function will receive a grid of width ```2 * col_range + 1```, centered on the cell that will be replaced by the output.
    pub col_range: usize,
    /// How the rule is supposed to handle cells at the edges of the state space.
    /// The first item describes how to handle trying to access rows out of range, the second columns out of range.
    pub boundaries: (super::BoundaryBehaviour, super::BoundaryBehaviour),
    /// The function that calculates the next state of a single cell based on its environment.
    ///
    /// Receives a grid of size ```2 * row_range + 1``` x ```2 * col_range + 1```. Must return a character.
    /// In the next iteration after applying this rule, the cell in the center of the received grid will contain the return value of this function.
    pub cell_transform: fn(&CellGrid) -> char,
}

impl Default for EnvironmentRule {
    fn default() -> Self {
        Self {
            row_range: Default::default(),
            col_range: Default::default(),
            boundaries: Default::default(),
            cell_transform: |_| ' ',
        }
    }
}

impl std::fmt::Debug for EnvironmentRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvironmentRule")
            .field("range_vert", &self.row_range)
            .field("range_hor", &self.col_range)
            .finish()
    }
}

impl super::Rule for EnvironmentRule {
    fn transform(&self, grid: &mut CellGrid) {
        let mut buffer = grid::Grid::new(2 * self.row_range + 1, 2 * self.col_range + 1);
        let (rows, cols) = grid.size();

        // correction factor to make sure no overflowing subtractions happen

        let mut res = CellGrid::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                for row_del in 0..=2 * self.row_range {
                    for col_del in 0..=2 * self.col_range {
                        // Calculate the index we are interested in.
                        let (mut t_row, mut t_col) = (
                            (row + row_del).wrapping_sub(self.row_range),
                            (col + col_del).wrapping_sub(self.col_range),
                        );

                        let mut done = false;

                        // If it is too large check the boundary condition.
                        // The < 0 case is handled because we are performing a wrapping sub.
                        // This might be error-prone if rows is close to the maximum value of a usize.
                        if t_col >= cols {
                            match self.boundaries.1 {
                                // Perdiodic: Take the modulus.
                                super::BoundaryBehaviour::Periodic => t_col %= cols,
                                // Symbol: Set the buffer to a fixed element.
                                super::BoundaryBehaviour::Symbol(symbol) => {
                                    buffer[row_del][col_del] = symbol;
                                    done = true;
                                }
                            }
                        }

                        // Do the same for rows. Doing rows later ensures the boundary symbol of rows takes precedence if need be.
                        if t_row >= rows {
                            match self.boundaries.0 {
                                // Perdiodic: Take the modulus.
                                super::BoundaryBehaviour::Periodic => t_row %= rows,
                                // Symbol: Set the buffer to a fixed element.
                                super::BoundaryBehaviour::Symbol(symbol) => {
                                    buffer[row_del][col_del] = symbol;
                                    done = true;
                                }
                            }
                        }

                        if !done {
                            buffer[row_del][col_del] = grid[t_row][t_col]
                        }
                    }
                }
                res[row][col] = (self.cell_transform)(&buffer);
            }
        }

        *grid = res;
    }
}
