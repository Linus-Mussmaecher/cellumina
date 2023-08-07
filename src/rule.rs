use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

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
                    chance: 0.3,
                    before: grid::grid![[' ']['F']],
                    after: grid::grid![['F'][' ']],
                },
                Pattern {
                    chance: 0.1,
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
        let (rows, cols) = grid.size();

        let mut clear_grid = grid::Grid::new(rows, cols);
        clear_grid.fill('*');

        let replacements = self
            .patterns
            .par_iter()
            .map(|pattern| {
                let mut partial_res = clear_grid.clone();
                for row in 0..rows {
                    'pattern_loop: for col in 0..cols {
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
                                if pattern.before[row_del][col_del] != '*'
                                    && grid[row + row_del][col + col_del]
                                        != pattern.before[row_del][col_del]
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
                                    partial_res[row + row_del][col + col_del] = rep;
                                }
                            }
                        }
                    }
                }
                partial_res
            })
            .reduce(
                || clear_grid.clone(),
                |mut grid_a, grid_b| {
                    for (index, cell) in grid_a.iter_mut().enumerate() {
                        if grid_b[index / cols][index % cols] != '*' {
                            *cell = grid_b[index / cols][index % cols];
                        }
                    }
                    grid_a
                },
            );

        let mut res = grid.clone();

        for (index, cell) in res.iter_mut().enumerate() {
            if replacements[index / cols][index % cols] != '*' {
                *cell = replacements[index / cols][index % cols];
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
    cell_transform: fn(&CellGrid) -> char,
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

impl Rule for EnvironmentRule {
    fn transform(&self, grid: &CellGrid) -> CellGrid {
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

        res
    }
}
