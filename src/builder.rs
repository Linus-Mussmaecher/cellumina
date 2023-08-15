use crate::{automaton, CellGrid};
use std::collections::HashMap;

use crate::rule;
/// Builder struct for [Automaton](automaton::Automaton).
///
/// Uses the builder pattern.
/// Create a builder using [Self::new()] and supply the following parameters:
///  -  An initial state from a text file, image file or other source.
///     If you supply no initial state, an empty grid of dimensions 10x10 will be used.
///  -  One or multiple [Rules](rule::Rule).
///     Supplying no rules will produce a static automaton.
///     Supplying multiple rules will combine them into a single [MultiRule](rule::MultiRule), i.e. they will all be applied each step in the order you passed them in.
///
///     Additionaly, you can add [rule::Pattern]s that will be added to an internal [Pattern Rule](rule::PatternRule).
///     In the building process, this [Pattern Rule](rule::PatternRule) will be added to the collection of rules supplied in other ways and be treated equally.
///     Therefore, supplying only patterns will create an automaton with only a single [Pattern Rule](rule::PatternRule).
///
///     Supplying Patters both by adding separate [Pattern Rule](rule::PatternRule)s and adding [rule::Pattern]s manually is not recommended, as this will create two PatternRules that need to be applied seperately and cannot be convoluted and parallelized.
/// -   One or multiple color mappings. These allow the state to be displayed or be converted into an image.
///     The colors are also used when reading in your initial state from an image.
/// -   Optionally, a time step to describe how often the automaton should transform itself using its rules.
///
/// Finally, the [Self::build()] function will consume this builder to create a [cellular automaton](automaton::Automaton).
///
/// If the created automaton is running on a fixed time step, it will not start counting until [automaton::Automaton::next_step] is called for the first time.
#[derive(Debug)]
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
    TextFile(Box<dyn AsRef<std::path::Path>>),
    ImageFile(Box<dyn AsRef<std::path::Path>>),
    ImageBuffer(image::ImageBuffer<image::Rgba<u8>, Vec<u8>>),
    Grid(CellGrid),
}

impl std::fmt::Debug for InitSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::TextFile(arg0) => f
                .debug_tuple("TextFile")
                .field(&(*arg0.as_ref()).as_ref().to_str())
                .finish(),
            Self::ImageFile(arg0) => f
                .debug_tuple("ImageFile")
                .field(&(*arg0.as_ref()).as_ref().to_str())
                .finish(),
            Self::ImageBuffer(arg0) => f.debug_tuple("ImageBuffer").field(arg0).finish(),
            Self::Grid(arg0) => f.debug_tuple("Grid").field(arg0).finish(),
        }
    }
}

impl AutomatonBuilder {
    /// Create a new [AutomatonBuilder] with no rules, state or time interval.
    pub fn new() -> Self {
        Self {
            pattern_rule: rule::PatternRule::new_empty(),
            rules: Vec::new(),
            source: InitSource::None,
            colors: HashMap::new(),
            step_mode: automaton::StepMode::Immediate,
        }
    }

    /// Set a minimum time step for the automaton.
    ///
    /// Setting a minimum time step causes the automatons [`next_step()`](automaton::Automaton::next_step) function to only perform a time step if a duration ```interval``` has elapsed since its previous invocation.
    /// This allows you to call ```next_step()``` every frame of your application, but invoke time steps with a much lower frame rate.
    /// Note that the return value of ```next_step()``` can be used to determine if the state has changed since last invocation and a redraw is neccessary.
    ///
    /// See also: [`automaton::Automaton::next_step()`].
    pub fn with_min_time_step(mut self, interval: std::time::Duration) -> Self {
        self.step_mode = automaton::StepMode::Limited { interval };
        self
    }

    /// Use a text file to supply the initial state of the automaton.
    ///
    /// The automaton will have as many rows as the file has lines, and as many columns as the longest line in the file is long.
    /// Will strip newlines.
    pub fn from_text_file(mut self, path: impl AsRef<std::path::Path> + 'static) -> Self {
        self.source = InitSource::TextFile(Box::new(path));
        self
    }

    /// Use an image file to supply the initial state of the automaton.
    ///
    /// The automatons dimensions (rows, columns) will be equal to the image dimensions (height, width).
    pub fn from_image_file(mut self, path: impl AsRef<std::path::Path> + 'static) -> Self {
        self.source = InitSource::ImageFile(Box::new(path));
        self
    }

    /// Use an image buffer to supply the initial state of the automaton.
    ///
    /// The automatons dimensions (rows, columns) will be equal to the image dimensions (height, width).
    pub fn from_image_buffer(
        mut self,
        buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Self {
        self.source = InitSource::ImageBuffer(buffer);
        self
    }

    /// Use an already prepared [CellGrid] as the initial state of the automaton.
    ///
    /// The automatons dimensions will be the dimensions of the grid.
    pub fn from_grid(mut self, grid: CellGrid) -> Self {
        self.source = InitSource::Grid(grid);
        self
    }

    /// Use a vector to supply the initial state of the automaton.
    /// The automaton will have as many columns as specified and as many rows as the vector can fill, ```ceil(vec.len() / columns)``` many.
    /// If the vector can't fully fill the last row, it will be padded with spaces.
    pub fn from_vec(mut self, mut vec: Vec<char>, columns: u32) -> Self {
        vec.resize(
            (columns * (vec.len() as u32 + columns - 1) / columns) as usize,
            ' ',
        );
        self.source = InitSource::Grid(grid::Grid::from_vec(vec, columns as usize));
        self
    }

    /// Adds a [Pattern](rule::Pattern) to this automaton that will be used for replacement each step.
    pub fn with_pattern(mut self, pattern: rule::Pattern) -> Self {
        self.pattern_rule.patterns.push(pattern);
        self
    }

    /// Adds multiple [Patterns](rule::Pattern) to this automaton that will be used for replacement each step.
    pub fn with_patterns(mut self, patterns: &[rule::Pattern]) -> Self {
        self.pattern_rule.patterns.extend(patterns.iter().cloned());
        self
    }

    /// Adds a rule to this automaton.
    ///
    /// Adding multiple rules will combine them into a single [MultiRule](rule::MultiRule) on construction.
    ///
    /// It is not suggested to use this function to add a [Pattern Rule](rule::PatternRule) and instead use [Self::with_pattern] or [Self::with_patterns].
    /// Only use this to add a [Pattern Rule](rule::PatternRule) when you have already constructed it elsewhere or reuse the same [Pattern Rule](rule::PatternRule) for multiple automata.
    pub fn with_rule(mut self, rule: impl rule::Rule + 'static) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    // TODO: rules from file

    /// Adds a color mapping to this automaton.
    /// Cells containing the character ```cell``` will be displayed as color ```color```.
    /// These colors are also used when converting to and from image buffers.
    pub fn with_color(mut self, cell: char, color: [u8; 4]) -> Self {
        self.colors.insert(cell, color);
        self
    }

    /// Adds multiple color mappings at once.
    /// Cells containing the character ```key``` will be displayed as color ```colors[key]```.
    /// These colors are also used when converting to and from image buffers.
    pub fn with_colors(mut self, colors: HashMap<char, [u8; 4]>) -> Self {
        self.colors.extend(colors);
        self
    }

    // TODO: colors from file

    /// Completes the build process and produces an [cellular automaton](automaton::Automaton) as specified.
    pub fn build(mut self) -> automaton::Automaton {
        automaton::Automaton {
            state: match std::mem::replace(&mut self.source, InitSource::None) {
                InitSource::None => grid::Grid::new(10, 10),
                InitSource::Grid(grid) => grid,
                InitSource::TextFile(path) => {
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
                InitSource::ImageBuffer(image) => self.buffer_to_grid(image),
                InitSource::ImageFile(path) => self.buffer_to_grid(
                    image::io::Reader::open(path.as_ref())
                        .expect("Could not read file.")
                        .decode()
                        .expect("Could not decode file.")
                        .into_rgba8(),
                ),
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
            last_step: None,
            colors: self.colors,
            manual_change: false,
        }
    }

    fn buffer_to_grid(
        &self,
        buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> crate::CellGrid {
        let mut grid = grid::Grid::new(
            buffer.dimensions().1 as usize,
            buffer.dimensions().0 as usize,
        );

        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                grid[row][col] = self
                    .colors
                    .iter()
                    .find_map(|(key, value)| {
                        if value == &buffer.get_pixel(col as u32, row as u32).0 {
                            Some(key)
                        } else {
                            None
                        }
                    })
                    .copied()
                    .unwrap_or(' ')
            }
        }

        grid
    }
}

impl Default for AutomatonBuilder {
    fn default() -> Self {
        Self::new()
    }
}
