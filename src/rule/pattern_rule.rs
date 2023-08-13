use super::Rule;
use crate::cell_state::CellGrid;
use rand::seq::SliceRandom;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone)]
pub struct PatternRule {
    pub(crate) patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub chance: f32,
    pub priority: f32,
    pub before: CellGrid,
    pub after: CellGrid,
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
    /// Create a new (empty) pattern rule.
    pub fn new_empty() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn new_sand() -> Self {
        Self {
            patterns: vec![
                Pattern {
                    before: grid::grid![['X'][' '][' ']],
                    after: grid::grid![[' '][' ']['X']],
                    priority: 1.0,
                    chance: 0.9,
                },
                Pattern {
                    before: grid::grid![['X'][' ']],
                    after: grid::grid![[' ']['X']],
                    priority: 0.5,
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

type ReplacementCollection = Vec<Vec<(f32, usize, usize, char)>>;

impl Rule for PatternRule {
    fn transform(&self, grid: &mut CellGrid) {
        let (rows, cols) = grid.size();

        let mut replacements: ReplacementCollection = self
            .patterns
            .par_iter()
            .filter_map(|pattern| {
                let mut partial_res = Vec::new();
                for row in 0..rows {
                    'inner_loop: for col in 0..cols {
                        let (p_rows, p_cols) = pattern.after.size();

                        // check if pattern would move out of bounds
                        if row + p_rows > rows
                            || col + p_cols > cols
                            // or immediately randomly stop to adhere to pattern chance
                            || rand::random::<f32>() > pattern.chance
                        {
                            continue 'inner_loop;
                        }

                        // check if pattern is applicable
                        for row_del in 0..p_rows {
                            for col_del in 0..p_cols {
                                if pattern.before[row_del][col_del] != '*'
                                    && grid[row + row_del][col + col_del]
                                        != pattern.before[row_del][col_del]
                                {
                                    continue 'inner_loop;
                                }
                            }
                        }

                        // if we arrive here, the pattern fits
                        let mut rep_group = Vec::new();
                        // push replacements as dictated by the pattern
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
                // only return partial result if it contains any elements
                if partial_res.is_empty() {
                    None
                } else {
                    Some(partial_res)
                }
            })
            .flatten()
            .collect();

        // shuffle the replacements
        replacements.shuffle(&mut rand::thread_rng());
        // then re-sort them by priority
        replacements.sort_by(|rule1, rule2| {
            if let Some(rep1) = rule1.first() {
                if let Some(rep2) = rule2.first() {
                    rep2.0
                        .partial_cmp(&rep1.0)
                        .unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    std::cmp::Ordering::Equal
                }
            } else {
                std::cmp::Ordering::Equal
            }
        });

        let mut mutated = grid::Grid::new(rows, cols);
        mutated.fill(false);

        for rep_group in replacements.iter() {
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
