mod environment_rule;
mod pattern_rule;

pub use environment_rule::EnvironmentRule;
pub use pattern_rule::Pattern;
pub use pattern_rule::PatternRule;

use crate::cell_state;

pub trait Rule {
    fn transform(&self, grid: &mut cell_state::CellGrid);
}

pub(crate) struct MultiRule {
    pub(crate) rules: Vec<Box<dyn Rule>>,
}

impl Rule for MultiRule {
    fn transform(&self, grid: &mut cell_state::CellGrid) {
        for rule in &self.rules {
            rule.transform(grid);
        }
    }
}
