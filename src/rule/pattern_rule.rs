use super::{EdgeBehaviour, Rule};
use crate::CellGrid;
use rand::seq::SliceRandom;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

/// A Pattern Rule works by looping over the current state and replacing every occurence of one or more certain patterns with another, equally sized pattern of characters.
///
/// For more information about how [Pattern]s are processed, see [Pattern].
#[derive(Debug, Clone)]
pub struct PatternRule {
    pub(crate) patterns: Vec<Pattern>,
    /// How the patterns in this rule will deal with the edges of the state space. Currently non-functional.
    pub(crate) edge_behaviour: EdgeBehaviour,
}

/// A pattern consists both of a grid of cells to search for and a grid of cells to replace it with.
///
/// The ```before``` pattern may contain wildcards ```*``` to match any character.
/// The ```after``` pattern may contain wildcards ```*``` to not mutate that cell and simply keep its previous value.
///
/// Whenever a pattern matches, the attribute might randomly be discarded instead of being applied.
/// The ```chance``` attribute describes the likelihood of the pattern being applied without discard, i.e. any value over ```1.0``` means the pattern will always be applied when it matches.
///
/// If multiple patterns are applicable within a time step, the one with higher priority will always be applied first.
/// Only if no cell concerning the second pattern has been mutated, the second pattern will apply also.
/// ```
/// use cellumina::rule::Rule;
/// let rule = cellumina::rule::PatternRule::from_patterns(
///     &[
///         cellumina::rule::Pattern{
///             chance: 1.0,
///             priority: 1.0,
///             before: grid::grid![['X'][' ']],
///             after: grid::grid![[' ']['X']],
///         },
///         cellumina::rule::Pattern{
///             chance: 1.0,
///             priority: 0.5,
///             before: grid::grid![[' ', 'X']['X', ' ']],
///             after: grid::grid![['X', 'X'][' ', ' ']],
///         },
///     ],
///     cellumina::rule::EdgeBehaviour::Show,
/// );
///
/// let mut grid = grid::grid![[' ', 'X']['X', ' '][' ', ' ']];
/// rule.transform(&mut grid);
/// assert_eq!(grid, grid::grid![[' ', ' '][' ', 'X']['X', ' ']]);
/// rule.transform(&mut grid);
/// assert_eq!(grid, grid::grid![[' ', ' '][' ', ' ']['X', 'X']]);
/// ```
#[derive(Debug, Clone)]
pub struct Pattern {
    /// The chance for the pattern to apply on a match.
    pub chance: f32,
    /// The priority of this pattern over others.
    pub priority: f32,
    /// The cell pattern to search for.
    pub before: CellGrid,
    /// The cell pattern it should be replaced with.
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
            edge_behaviour: EdgeBehaviour::Show,
        }
    }

    /// Create a new pattern rule from a set of patterns.
    pub fn from_patterns(rules: &[Pattern], edge_behaviour: EdgeBehaviour) -> Self {
        Self {
            patterns: rules.to_vec(),
            edge_behaviour,
        }
    }
}

/// A collection of replacement actions, containing a priority, a position (row/column) and a placement character.
/// A pattern will always produce such a collection of replacements belonging together.
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

                        // possibly immediately randomly stop to adhere to pattern chance
                        if rand::random::<f32>() > pattern.chance {
                            continue 'inner_loop;
                        }

                        // check if pattern is applicable
                        for row_del in 0..p_rows {
                            for col_del in 0..p_cols {
                                if pattern.before[row_del][col_del] != '*'
                                    && grid
                                        .get(row + row_del, col + col_del)
                                        .copied()
                                        // to get the char, try to get it normally. If we would move out of bounds, check edge behaviour
                                        .unwrap_or_else(|| match self.edge_behaviour {
                                            // if set to Wrap, wrap around the cell grid
                                            EdgeBehaviour::Wrap => {
                                                grid[(row + rows + row_del) % rows]
                                                    [(col + cols + col_del) % cols]
                                            }
                                            // if set to show, display an underscore to indicate a wall
                                            EdgeBehaviour::Show => '_',
                                        })
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
                                // make sure to not replace wild cards, and check edge behaviour
                                if rep != '*'
                                // only push changes if within bounds, or edge behaviours is set to wrap
                                    && ((row + row_del < rows && col + col_del < cols)
                                        || match self.edge_behaviour {
                                            EdgeBehaviour::Wrap => true,
                                            EdgeBehaviour::Show => false,
                                        })
                                {
                                    // not that we still apply a modulus to the coordinates, so if this was let through by wrap behaviour, we are not actually moving out of bounds
                                    rep_group.push((
                                        pattern.priority,
                                        (row + row_del) % rows,
                                        (col + col_del) % cols,
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
