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
///     environment_size: [1,1,1,1],
///     row_boundary: cellumina::rule::BoundaryBehaviour::Periodic,
///     col_boundary: cellumina::rule::BoundaryBehaviour::Periodic,
///     cell_transform: |env: &cellumina::CellGrid| match env
///     // Iterate over neighbors.
///         .iter()
///         .enumerate()
///         .map(|val| match val {
///             // The cell we are transforming does not get counted.
///             (4, 1) => 0,
///             // Any cell containing an 1 counts for 1 (alive).
///             (_, 1) => 1,
///             // Any cell containing any other entry (only 0 in our initial configuration) counts as 0 (dead).
///             _ => 0,
///         })
///         // Sum over these 9 values...
///         .sum()
///         // ... and map the sum to the new enty of our cell:
///     {
///         // 2 neighbors: The cell keeps its state.
///         2 => env[1][1],
///         // 3 neighbors: The cell gets born.
///         3 => 1,
///         // 0, 1 or more than 3 neighbors: The cell dies.
///         _ => 0,
///     },
/// };
/// let mut grid = grid::grid![[0, 0, 1, 0, 0][0, 0, 1,0, 0][0, 0, 0, 0, 0][0, 0, 1, 0, 0][0, 0, 1, 0, 0]];
/// rule.transform(&mut grid);
/// assert_eq!(
///     grid,
///     grid::grid![[0, 1, 1, 1, 0][0, 0, 0,0, 0][0, 0, 0, 0, 0][0, 0, 0, 0, 0][0, 1, 1, 1, 0]]
/// );
/// rule.transform(&mut grid);
/// rule.transform(&mut grid);
/// rule.transform(&mut grid);
/// assert_eq!(
///     grid,
///     grid::grid![[0, 1, 0, 1, 0][0, 0, 1,0, 0][0, 0, 0, 0, 0][0, 0, 1, 0, 0][0, 1, 0, 1, 0]]
/// );
/// ```
#[derive(Clone, Copy)]
pub struct EnvironmentRule {
    /// The distance the considered environment extends from the cell to be set, in order ```[top, right, bottom, left]```.
    ///
    /// Your cell_transform function will receive a grid of size ```(top + bottom + 1) * (left + right + 1)``` to calculate the next state of the cell in the middle.
    ///
    /// ```text
    ///  +---------------+
    ///  |       |       |
    ///  |      top      |
    ///  |       |       |
    ///  |--left-C-right-|
    ///  |       |       |
    ///  |     bottom    |
    ///  |       |       |
    ///  +---------------+
    ///
    /// ```
    pub environment_size: [usize; 4],
    /// Behaviour of this rule when encountering cases in which the environment of a cell contains rows that go out of bounds of the state grid.
    pub row_boundary: super::BoundaryBehaviour,
    /// Behaviour of this rule when encountering cases in which the environment of a cell contains columns that go out of bounds of the state grid.
    pub col_boundary: super::BoundaryBehaviour,
    /// The function that calculates the next state of a single cell based on its environment.
    ///
    /// Receives a grid of size ```(top + bottom + 1) * (left + right + 1)```, where ```[top, right, bottom, left]``` is the ```enviroment_size```.
    /// Must return a character.
    /// In the next iteration after applying this rule, the cell at position ```[top][left]```, with ```[0][0]``` being the top right, of the received grid will contain the return value of this function.
    pub cell_transform: fn(&CellGrid) -> u8,
}

impl Default for EnvironmentRule {
    fn default() -> Self {
        Self {
            environment_size: [1, 1, 1, 1],
            row_boundary: Default::default(),
            col_boundary: Default::default(),
            cell_transform: |_| 0,
        }
    }
}

impl std::fmt::Debug for EnvironmentRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvironmentRule")
            .field("environment_size", &self.environment_size)
            .field("row_boundary", &self.row_boundary)
            .field("col_boundary", &self.col_boundary)
            //.field("cell_transform", &self.cell_transform)
            .finish()
    }
}

impl super::Rule for EnvironmentRule {
    fn transform(&self, grid: &mut CellGrid) {
        let mut buffer = grid::Grid::new(
            self.environment_size[0] + self.environment_size[2] + 1,
            self.environment_size[1] + self.environment_size[3] + 1,
        );
        let (rows, cols) = grid.size();

        // correction factor to make sure no overflowing subtractions happen

        let mut res = CellGrid::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                for row_del in 0..=(self.environment_size[0] + self.environment_size[2]) {
                    for col_del in 0..=(self.environment_size[1] + self.environment_size[3]) {
                        // Calculate the index we are interested in.
                        let (mut t_row, mut t_col) = (
                            (row + row_del).wrapping_sub(self.environment_size[0]),
                            (col + col_del).wrapping_sub(self.environment_size[3]),
                        );

                        let mut done = false;

                        // If it is too large check the boundary condition.
                        // The < 0 case is handled because we are performing a wrapping sub.
                        // This might be error-prone if rows is close to the maximum value of a usize.
                        if t_col >= cols {
                            match self.col_boundary {
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
                            match self.row_boundary {
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
