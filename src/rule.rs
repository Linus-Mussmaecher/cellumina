use rand::seq::SliceRandom;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::cell_state::CellGrid;

pub trait Rule {
    fn transform(&self, grid: &mut CellGrid);
}

pub struct MultiRule {
    rules: Vec<Box<dyn Rule>>,
}

impl Rule for MultiRule {
    fn transform(&self, grid: &mut CellGrid) {
        for rule in &self.rules {
            rule.transform(grid);
        }
    }
}

pub struct PatternRule {
    patterns: Vec<Pattern>,
}

pub struct Pattern {
    chance: f32,
    priority: f32,
    before: CellGrid,
    after: CellGrid,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            chance: 1.,
            priority: 0.,
            before: grid::grid![['*']],
            after: grid::grid![['*']],
        }
    }
}

impl PatternRule {
    pub fn new_sand() -> Self {
        Self {
            patterns: vec![
                Pattern {
                    before: grid::grid![['X'][' '][' ']],
                    after: grid::grid![[' '][' ']['X']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![['X'][' ']],
                    after: grid::grid![[' ']['X']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![['X', ' ']['X', ' ']],
                    after: grid::grid![[' ', ' ']['X', 'X']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![[' ', 'X'][' ', 'X']],
                    after: grid::grid![[' ', ' ']['X', 'X']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![['X', ' ', ' ']['X', 'X', ' ']],
                    after: grid::grid![[' ', ' ', ' ']['X', 'X', 'X']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![[' ', ' ', 'X'][' ', 'X', 'X']],
                    after: grid::grid![[' ', ' ', ' ']['X', 'X', 'X']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.3,
                    before: grid::grid![[' ']['F']],
                    after: grid::grid![['F'][' ']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.1,
                    before: grid::grid![['F'][' ']],
                    after: grid::grid![[' ']['F']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['X']['F']],
                    after: grid::grid![['F']['F']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['X', 'F']],
                    after: grid::grid![['F', 'F']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['F', 'X']],
                    after: grid::grid![['F', 'F']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['F']['X']],
                    after: grid::grid![['F']['F']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['X', '*']['*', 'F']],
                    after: grid::grid![['F', '*']['*', '*']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['*', 'X']['F', '*']],
                    after: grid::grid![['*', 'F']['*', '*']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['*', 'F']['X', '*']],
                    after: grid::grid![['*', '*']['F', '*']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.8,
                    before: grid::grid![['F', '*']['*', 'X']],
                    after: grid::grid![['*', '*']['*', 'F']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.03,
                    before: grid::grid![['F']],
                    after: grid::grid![['A']],
                    priority: 1.,
                },
                Pattern {
                    before: grid::grid![['A'][' ']],
                    after: grid::grid![[' ']['A']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![['A', ' ']['A', ' ']],
                    after: grid::grid![[' ', '*']['A', 'A']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![[' ', 'A'][' ', 'A']],
                    after: grid::grid![['*', ' ']['A', 'A']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![['A']['F']],
                    after: grid::grid![['F']['A']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![['A']['X']],
                    after: grid::grid![[' ']['F']],
                    ..Default::default()
                },
                Pattern {
                    before: grid::grid![['X']['A']],
                    after: grid::grid![['F']['*']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.1,
                    before: grid::grid![[' ', 'F']],
                    after: grid::grid![['F', ' ']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.1,
                    before: grid::grid![['F', ' ']],
                    after: grid::grid![[' ', 'F']],
                    ..Default::default()
                },
                Pattern {
                    chance: 0.5,
                    before: grid::grid![['*']['S']],
                    after: grid::grid![['F']['S']],
                    ..Default::default()
                },
            ],
        }
    }
}

impl Rule for PatternRule {
    fn transform(&self, grid: &mut CellGrid) {
        let (rows, cols) = grid.size();

        let mut replacements = Vec::new();

        self.patterns
            .par_iter()
            .map(|pattern| {
                let mut partial_res = Vec::new();
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
                        let mut rep_group = Vec::new();
                        // mutate the cells as described by this pattern
                        for row_del in 0..p_rows {
                            for col_del in 0..p_cols {
                                let rep = pattern.after[row_del][col_del];
                                if rep != '*' {
                                    rep_group.push((
                                        pattern.priority,
                                        row + row_del,
                                        col + col_del,
                                        rep,
                                    ));
                                }
                            }
                        }
                        partial_res.push(rep_group);
                    }
                }
                partial_res
            })
            .collect_into_vec(&mut replacements);

        replacements.shuffle(&mut rand::thread_rng());
        replacements.sort_by(|rule1, rule2| {
            if rule1.is_empty() || rule2.is_empty() {
                return std::cmp::Ordering::Equal;
            }
            let rep1 = *rule1.first().unwrap().first().unwrap();
            let rep2 = *rule2.first().unwrap().first().unwrap();
            rep2.0
                .partial_cmp(&rep1.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut mutated = grid::Grid::new(rows, cols);
        mutated.fill(false);

        for rep_group in replacements.iter().flatten() {
            if rep_group
                .iter()
                .all(|(_, row, col, _)| !mutated[*row][*col])
            {
                for (_, row, col, rep) in rep_group.iter().copied() {
                    grid[row][col] = rep;
                    mutated[row][col] = true;
                }
            }
        }
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
