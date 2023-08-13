mod environment_rule;
mod pattern_rule;

use super::CellGrid;
pub use environment_rule::EnvironmentRule;
pub use pattern_rule::Pattern;
pub use pattern_rule::PatternRule;

pub trait Rule {
    fn transform(&self, grid: &mut CellGrid);
}

pub(crate) struct MultiRule {
    pub(crate) rules: Vec<Box<dyn Rule>>,
}

impl Rule for MultiRule {
    fn transform(&self, grid: &mut CellGrid) {
        for rule in &self.rules {
            rule.transform(grid);
        }
    }
}
