mod environment_rule;
mod pattern_rule;

use std::fmt::Debug;
use std::fmt::Display;

use super::CellGrid;
pub use environment_rule::EnvironmentRule;
pub use pattern_rule::Pattern;
pub use pattern_rule::PatternRule;

/// A rule describes a transition from one state of a cellular automaton to the next.
pub trait Rule: Debug {
    /// Transforms the passed cell grid according to this transformation rule.
    /// Transformation happens in-place.
    fn transform(&self, grid: &mut CellGrid);
}

/// A multi rule consists of multiple rules. Each rule will be applied in order, and the result of the final application is the result of the multi rule.
#[derive(Debug)]
pub struct MultiRule {
    /// The collection of rules to be applied in order.
    pub(crate) rules: Vec<Box<dyn Rule>>,
}

impl Rule for MultiRule {
    fn transform(&self, grid: &mut CellGrid) {
        for rule in &self.rules {
            rule.transform(grid);
        }
    }
}

/// Describes how Rules, specifically [EnvironmentRule] and [PatternRule], deal with the boundaries of the state grid.
#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum BoundaryBehaviour {
    #[default]
    /// When trying to get a cell from an index outside of the state grid, wrap around
    Periodic,
    /// When trying to get a cell from outside the state grid, return '_' to indicate a wall.
    /// [PatternRule] will simply not check subareas that leave the state grid.
    Symbol(char),
}

impl Display for BoundaryBehaviour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundaryBehaviour::Periodic => write!(f, "Periodic"),
            BoundaryBehaviour::Symbol(symbol) => write!(f, "Symbol:{symbol}"),
        }
    }
}

impl From<&str> for BoundaryBehaviour {
    fn from(value: &str) -> Self {
        match value {
            "Periodic" => Self::Periodic,
            value => {
                let parts = value.split(':').collect::<Vec<&str>>();
                if parts[0] == "Symbol" {
                    Self::Symbol(parts[1].as_bytes()[0] as char)
                } else {
                    Self::Symbol(' ')
                }
            }
        }
    }
}
