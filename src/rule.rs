use std::{collections::HashMap, usize};

use crate::cell_state::CellGrid;

pub trait Rule {
    fn transform(&self, grid: &CellGrid) -> CellGrid;
}
pub struct CountingRule {
    /// The vertical range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    range_vert: usize,
    /// The horizontal range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    range_hor: usize,
    /// The environemnt rules. Need to be complete.
    rules: HashMap<u32, char>,
    /// The count values of the different chars.
    counts: HashMap<char, u32>,
}

impl CountingRule {
    pub fn new_gol() -> Self {
        Self {
            range_vert: 1,
            range_hor: 1,
            rules: HashMap::from([
                (0, ' '),
                (1, ' '),
                (3, 'X'),
                (4, ' '),
                (5, ' '),
                (6, ' '),
                (7, ' '),
                (8, ' '),
            ]),
            counts: HashMap::from([(' ', 0), ('X', 1)]),
        }
    }
}

impl Rule for CountingRule {
    fn transform(&self, grid: &CellGrid) -> CellGrid {
        let (h, w) = grid.size();

        // correction factor to make sure no overflowing subtractions happen

        let mut res = CellGrid::new(h, w);

        for row in 0..h {
            for col in 0..w {
                let mut count = 0;
                for row_del in 0..=2 * self.range_vert {
                    for col_del in 0..=2 * self.range_hor {
                        if row_del == self.range_vert && col_del == self.range_hor {
                            continue;
                        }
                        count += self
                            .counts
                            .get(
                                &grid[(row + h + row_del - self.range_vert) % h]
                                    [(col + w + col_del - self.range_hor) % w],
                            )
                            .copied()
                            .unwrap_or(0);
                    }
                }
                res[row][col] = self.rules.get(&count).copied().unwrap_or(grid[row][col]);
            }
        }

        res
    }
}

pub struct EnvironmentRule {
    /// The vertical range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    range_vert: usize,
    /// The horizontal range of an environment, extending in both direction from the cell to be transformed.
    /// Contract: (2 * rows + 1) * (2 * columns + 1)= S.
    range_hor: usize,
    /// The environemnt rules. Need to be complete.
    cell_transform: fn(&[char]) -> char,
}

impl EnvironmentRule {
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
                2 => env[4],
                3 => 'X',
                _ => ' ',
            },
        }
    }
}

impl Rule for EnvironmentRule {
    fn transform(&self, grid: &CellGrid) -> CellGrid {
        let mut buffer = Vec::with_capacity(self.range_hor * self.range_vert);
        let (h, w) = grid.size();

        // correction factor to make sure no overflowing subtractions happen

        let mut res = CellGrid::new(h, w);

        for row in 0..h {
            for col in 0..w {
                for row_del in 0..=2 * self.range_vert {
                    for col_del in 0..=2 * self.range_hor {
                        buffer.push(
                            grid[(row + h + row_del - self.range_vert) % h]
                                [(col + w + col_del - self.range_hor) % w],
                        );
                    }
                }
                res[row][col] = (self.cell_transform)(&buffer);
                buffer.clear();
            }
        }

        res
    }
}
