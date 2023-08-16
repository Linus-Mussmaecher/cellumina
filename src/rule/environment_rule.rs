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
///     range_vert: 1,
///     edge_behaviour: cellumina::rule::EdgeBehaviour::Wrap,
///     range_hor: 1,
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
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    pub range_vert: usize,
    /// The horizontal range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    pub range_hor: usize,
    /// How the rule is supposed to handle cells at the edges of the state space.
    pub edge_behaviour: EdgeBehaviour,
    /// The environemnt rules. Need to be complete.
    pub cell_transform: fn(&CellGrid) -> char,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum EdgeBehaviour {
    #[default]
    Wrap,
    Show,
}

impl Default for EnvironmentRule {
    fn default() -> Self {
        Self {
            range_vert: Default::default(),
            range_hor: Default::default(),
            edge_behaviour: Default::default(),
            cell_transform: |_| ' ',
        }
    }
}

impl std::fmt::Debug for EnvironmentRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvironmentRule")
            .field("range_vert", &self.range_vert)
            .field("range_hor", &self.range_hor)
            .finish()
    }
}

impl super::Rule for EnvironmentRule {
    fn transform(&self, grid: &mut CellGrid) {
        let mut buffer = grid::Grid::new(2 * self.range_vert + 1, 2 * self.range_hor + 1);
        let (rows, cols) = grid.size();

        // correction factor to make sure no overflowing subtractions happen

        let mut res = CellGrid::new(rows, cols);

        for row in 0..rows {
            for col in 0..cols {
                for row_del in 0..=2 * self.range_vert {
                    for col_del in 0..=2 * self.range_hor {
                        // Fill the buffer with values from the grid
                        buffer[row_del][col_del] = grid
                            // try to get normally
                            .get(
                                (row + row_del).wrapping_sub(self.range_vert),
                                (col + col_del).wrapping_sub(self.range_hor),
                            )
                            .copied()
                            // if outside of grid, check edge behavior
                            .unwrap_or_else(|| match self.edge_behaviour {
                                // Wrap: Do a modulus calculation to get from the other side of the grid
                                EdgeBehaviour::Wrap => {
                                    grid[(row + rows + row_del - self.range_vert) % rows]
                                        [(col + cols + col_del - self.range_hor) % cols]
                                }
                                // Show: Show 'Outside Of Grid'-Cells as Underscore.
                                EdgeBehaviour::Show => '_',
                            });
                    }
                }
                res[row][col] = (self.cell_transform)(&buffer);
            }
        }

        *grid = res;
    }
}
