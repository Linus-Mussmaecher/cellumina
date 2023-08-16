mod environment_rule;
mod pattern_rule;

use std::fmt::Debug;

use super::CellGrid;
pub use environment_rule::EdgeBehaviour;
pub use environment_rule::EnvironmentRule;
pub use pattern_rule::Pattern;
pub use pattern_rule::PatternRule;

/// A rule describes a transition from one state of a cellular automaton to the next.
pub trait Rule: Debug {
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
