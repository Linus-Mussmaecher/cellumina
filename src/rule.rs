use std::{collections::HashMap, usize};

use crate::cell_state::CellGrid;

pub trait Rule {
    fn transform(&self, grid: &CellGrid) -> CellGrid;
}

pub struct MultiRule {
    rules: Vec<Box<dyn Rule>>,
}

impl Rule for MultiRule {
    fn transform(&self, grid: &CellGrid) -> CellGrid {
        let mut res = grid.clone();
        for rule in &self.rules {
            res = rule.transform(&res);
        }
        res
    }
}

pub struct PatternRule {
    patterns: Vec<Pattern>,
}

pub struct Pattern {
    chance: f32,
    before: CellGrid,
    after: CellGrid,
}

impl PatternRule {
    pub fn new_sand() -> Self {
        Self {
            patterns: vec![
                Pattern {
                    chance: 1.,
                    before: grid::grid![['X'][' '][' ']],
                    after: grid::grid![[' '][' ']['X']],
                },
                Pattern {
                    chance: 1.,
                    before: grid::grid![['X'][' ']],
                    after: grid::grid![[' ']['X']],
                },
                Pattern {
                    chance: 1.,
                    before: grid::grid![['X', ' ']['X', ' ']],
                    after: grid::grid![[' ', ' ']['X', 'X']],
                },
                Pattern {
                    chance: 1.,
                    before: grid::grid![[' ', 'X'][' ', 'X']],
                    after: grid::grid![[' ', ' ']['X', 'X']],
                },
                Pattern {
                    chance: 1.,
                    before: grid::grid![['X', ' ', ' ']['X', 'X', ' ']],
                    after: grid::grid![[' ', ' ', ' ']['X', 'X', 'X']],
                },
                Pattern {
                    chance: 1.,
                    before: grid::grid![[' ', ' ', 'X'][' ', 'X', 'X']],
                    after: grid::grid![[' ', ' ', ' ']['X', 'X', 'X']],
                },
                Pattern {
                    chance: 0.6,
                    before: grid::grid![['F'][' ']],
                    after: grid::grid![[' ']['F']],
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['X', 'F']],
                    after: grid::grid![['F', 'F']],
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['F', 'X']],
                    after: grid::grid![['F', 'F']],
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['F']['X']],
                    after: grid::grid![['F']['F']],
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['X', '*']['*', 'F']],
                    after: grid::grid![['F', '*']['*', '*']],
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['*', 'X']['F', '*']],
                    after: grid::grid![['*', 'F']['*', '*']],
                },
                Pattern {
                    chance: 0.1,
                    before: grid::grid![['F']],
                    after: grid::grid![[' ']],
                },
                Pattern {
                    chance: 0.1,
                    before: grid::grid![[' ', 'F']],
                    after: grid::grid![['F', ' ']],
                },
                Pattern {
                    chance: 0.1,
                    before: grid::grid![['F', ' ']],
                    after: grid::grid![[' ', 'F']],
                },
                Pattern {
                    chance: 0.05,
                    before: grid::grid![['*']['S']],
                    after: grid::grid![['F']['S']],
                },
            ],
        }
    }
}

impl Rule for PatternRule {
    fn transform(&self, grid: &CellGrid) -> CellGrid {
        let mut res = grid.clone();
        let mut mutated = grid::Grid::new(grid.rows(), grid.cols());
        mutated.fill(false);

        let (rows, cols) = grid.size();

        for row in 0..rows {
            for col in 0..cols {
                'pattern_loop: for pattern in self.patterns.iter() {
                    let (p_rows, p_cols) = pattern.after.size();

                    // check if pattern would move out of bounds
                    if row + p_rows > rows
                        || col + p_cols > cols
                        || rand::random::<f32>() > pattern.chance
                    {
                        continue 'pattern_loop;
                    }

                    // check if pattern is applicable
                    for row_del in 0..p_rows {
                        for col_del in 0..p_cols {
                            if (
                                // check if pattern and grid agree on this cell
                                pattern.before[row_del][col_del] != '*'
                                && grid[row + row_del][col + col_del]
                                    != pattern.before[row_del][col_del])
                                // check if the cells this pattern wants to mutate are still mutatetable
                                || (pattern.after[row_del][col_del] != '*'
                                    && mutated[row + row_del][col + col_del])
                            {
                                continue 'pattern_loop;
                            }
                        }
                    }

                    // if we arrive here, the pattern fits (first check) and the cell are still free to mutate this step (second & third check)

                    // mutate the cells as described by this pattern
                    for row_del in 0..p_rows {
                        for col_del in 0..p_cols {
                            let rep = pattern.after[row_del][col_del];
                            if rep != '*' {
                                res[row + row_del][col + col_del] = rep;
                                mutated[row + row_del][col + col_del] = true;
                            }
                        }
                    }
                }
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
