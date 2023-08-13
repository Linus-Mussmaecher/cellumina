use crate::automaton;
use std::collections::HashMap;

use crate::rule;
/// Builder struct for [automaton::Automaton].
pub struct AutomatonBuilder {
    pattern_rule: rule::PatternRule,
    rules: Vec<Box<dyn rule::Rule>>,
    source: InitSource,
    colors: HashMap<char, [u8; 4]>,
    step_mode: automaton::StepMode,
    // TODO: EdgeBehaviour
}

enum InitSource {
    None,
    File(Box<dyn AsRef<std::path::Path>>),
    Image(image::ImageBuffer<image::Rgba<u8>, Vec<u8>>),
}

impl AutomatonBuilder {
    pub fn new() -> Self {
        Self {
            pattern_rule: rule::PatternRule::new_empty(),
            rules: Vec::new(),
            source: InitSource::None,
            colors: HashMap::new(),
            step_mode: automaton::StepMode::Immediate,
        }
    }

    pub fn with_time_step(mut self, interval: std::time::Duration) -> Self {
        self.step_mode = automaton::StepMode::Timed {
            interval,
            last_step: std::time::Instant::now(),
        };
        self
    }

    pub fn from_file(mut self, path: impl AsRef<std::path::Path> + 'static) -> Self {
        self.source = InitSource::File(Box::new(path));
        self
    }

    pub fn from_image_buffer(
        mut self,
        buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Self {
        self.source = InitSource::Image(buffer);
        self
    }

    pub fn with_pattern(mut self, pattern: rule::Pattern) -> Self {
        self.pattern_rule.patterns.push(pattern);
        self
    }

    pub fn with_rule(mut self, rule: impl rule::Rule + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    // TODO: rules from file

    pub fn with_color(mut self, cell: char, color: [u8; 4]) -> Self {
        self.colors.insert(cell, color);
        self
    }

    pub fn with_colors(mut self, colors: HashMap<char, [u8; 4]>) -> Self {
        self.colors.extend(colors);
        self
    }
    // TODO: colors from file

    pub fn build(mut self) -> automaton::Automaton {
        automaton::Automaton {
            state: match self.source {
                InitSource::None => grid::Grid::new(10, 10),
                InitSource::File(path) => {
                    // read file
                    let content =
                        std::fs::read_to_string(path.as_ref()).expect("Could not read file.");
                    // split into lines
                    let lines: Vec<&str> = content.split('\n').collect();
                    // get number of columns (chars in largest line)
                    // subtracting one from each line because of leftover newline
                    let cols = lines
                        .iter()
                        .map(|line| line.len().saturating_sub(1))
                        .max()
                        .unwrap_or_default();

                    // create grid to hold data
                    let mut grid = grid::Grid::<char>::new(0, cols);

                    // iterate over lines and add them to the grid
                    for line in lines {
                        // create char vector
                        let mut chars: Vec<char> = line.replace('\r', "").chars().collect();
                        // make sure vector is neither to large nor to small
                        chars.resize(cols, ' ');
                        // push to the grid
                        grid.push_row(chars);
                    }

                    grid
                }
                InitSource::Image(image) => {
                    todo!()
                }
            },
            rules: {
                if !self.pattern_rule.patterns.is_empty() {
                    self.rules.push(Box::new(self.pattern_rule));
                }

                if self.rules.len() == 1 {
                    self.rules.pop().unwrap()
                } else {
                    Box::new(rule::MultiRule { rules: self.rules })
                }
            },
            step_mode: self.step_mode,
            colors: self.colors,
        }
    }
}

impl Default for AutomatonBuilder {
    fn default() -> Self {
        Self::new()
    }
}
