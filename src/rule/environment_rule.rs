use crate::cell_state;

pub struct EnvironmentRule {
    /// The vertical range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    range_vert: usize,
    /// The horizontal range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    range_hor: usize,
    /// The environemnt rules. Need to be complete.
    cell_transform: fn(&cell_state::CellGrid) -> char,
}

impl EnvironmentRule {
    #[allow(dead_code)]
    pub fn new_gol() -> Self {
        Self {
            range_vert: 1,
            range_hor: 1,
            cell_transform: |env| match env
                .iter()
                .enumerate()
                .map(|val| match val {
                    (4, 'X') => 0,
                    (_, 'X') => 1,
                    _ => 0,
                })
                .sum()
            {
                2 => env[1][1],
                3 => 'X',
                _ => ' ',
            },
        }
    }
}

impl super::Rule for EnvironmentRule {
    fn transform(&self, grid: &mut cell_state::CellGrid) {
        let mut buffer = grid::Grid::new(2 * self.range_vert + 1, 2 * self.range_hor + 1);
        let (h, w) = grid.size();

        // correction factor to make sure no overflowing subtractions happen

        let mut res = cell_state::CellGrid::new(h, w);

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
