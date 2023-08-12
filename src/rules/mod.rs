pub mod environment_rule;
pub mod pattern_rule;

use crate::cell_state;

pub trait Rule {
    fn transform(&self, grid: &mut cell_state::CellGrid);
}

pub struct MultiRule {
    rules: Vec<Box<dyn Rule>>,
}

impl Rule for MultiRule {
    fn transform(&self, grid: &mut cell_state::CellGrid) {
        for rule in &self.rules {
            rule.transform(grid);
        }
    }
}
