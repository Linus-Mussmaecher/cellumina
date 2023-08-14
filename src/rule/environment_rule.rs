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
pub struct EnvironmentRule {
    /// The vertical range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    pub range_vert: usize,
    /// The horizontal range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    pub range_hor: usize,
    /// The environemnt rules. Need to be complete.
    pub cell_transform: fn(&CellGrid) -> char,
}

impl super::Rule for EnvironmentRule {
    fn transform(&self, grid: &mut CellGrid) {
        let mut buffer = grid::Grid::new(2 * self.range_vert + 1, 2 * self.range_hor + 1);
        let (h, w) = grid.size();

        // correction factor to make sure no overflowing subtractions happen

        let mut res = CellGrid::new(h, w);

        for row in 0..h {
            for col in 0..w {
                for row_del in 0..=2 * self.range_vert {
                    for col_del in 0..=2 * self.range_hor {
                        buffer[row_del][col_del] = grid[(row + h + row_del - self.range_vert) % h]
                            [(col + w + col_del - self.range_hor) % w];
                    }
                }
                res[row][col] = (self.cell_transform)(&buffer);
            }
        }

        *grid = res;
    }
}
