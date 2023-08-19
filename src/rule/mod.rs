mod environment_rule;
mod pattern_rule;

use std::fmt::Debug;

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

#[derive(Clone, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum EdgeBehaviour {
    #[default]
    /// When trying to get a cell from an index outside of the state space, wrap around
    Wrap,
    /// When trying to get a cell from outside the state space, return '_' to indicate a wall.
    /// PatternRules will not check subareas that leave the state space.
    Stop,
}
