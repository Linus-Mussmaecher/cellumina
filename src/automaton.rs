use std::{collections::HashMap, time};

use crate::{error::CelluminaError, rule, CellGrid};

/// A struct that represents the current state and rule set of a cellular automaton.
/// A cellular automaton has a state consisting of a (finite) character grid and a set of rules that describes how to process this grid to get the next state.
#[derive(Debug)]
pub struct Automaton {
    /// The current state of the automaton.
    pub(super) state: CellGrid,
    /// The rule set of the automaton.
    pub(super) rules: Box<dyn rule::Rule>,
    /// How often and on what conditions this automaton applies its rule set to its state to get to the next step.
    pub(super) step_mode: StepMode,
    /// The colors this automaton uses to convert itself to an image.
    pub(super) colors: HashMap<char, [u8; 4]>,
    /// The time at which the automaton was created or the last step was performed.
    pub(super) last_step: Option<time::Instant>,
    /// Wether a manual state change was performed between the current stept and the previous one.
    pub(super) manual_change: bool,
}

/// Describes how often an [Automaton] executes its time step.
#[derive(Clone, Copy, Debug)]
pub(super) enum StepMode {
    /// Time steps are performed on every call of the [Automaton::next_step] function.
    Immediate,
    /// Timed steps are performed every interval, but at most once per call of [Automaton::next_step].
    Limited { interval: time::Duration },
}

impl Automaton {
    /// Turns this automatons current state grid into an image buffer.
    pub fn create_image_buffer(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        image::ImageBuffer::from_fn(
            self.state.size().1 as u32,
            self.state.size().0 as u32,
            |col, row| {
                image::Rgba(
                    self.colors
                        .get(&self.state[row as usize][col as usize])
                        .copied()
                        .unwrap_or([0; 4]),
                )
            },
        )
    }

    /// Returns the dimensions of this automaton's state grid as a tuple, first are the number of rows (height), then the number of columns (width).
    /// The reason for this order is the column-major layout of the underlying [grid::Grid] state representation.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.state.size().0 as u32, self.state.size().1 as u32)
    }

    /// Sets the cell at the specified indices to the specified character.
    /// ## Error
    /// When the given index is out of bounds.
    pub fn set_cell(&mut self, row: u32, col: u32, new_val: char) -> Result<(), CelluminaError> {
        if row >= self.state.size().0 as u32 || col >= self.state.size().1 as u32 {
            Err(CelluminaError::IndexOutOfBoundsError(
                row,
                col,
                self.state.size().0 as u32,
                self.state.size().1 as u32,
            ))
        } else {
            self.manual_change = self.state[row as usize][col as usize] != new_val;
            self.state[row as usize][col as usize] = new_val;
            Ok(())
        }
    }

    /// Checks if and how many time steps should currently be executed and performs them.
    /// A time step consists of applying this automatons rule to its state, thus transforming the state.
    /// ## Returns
    /// Wether or not the state has changed since the last invocation of [next_step](Automaton::next_step()), either because a time step was performed or by manual interaction between steps.
    pub fn next_step(&mut self) -> bool {
        // if the automaton has just started, set last step for the first time
        if self.last_step.is_none() {
            self.last_step = Some(time::Instant::now());
        }
        // set manual change to false, then return its previous state and OR it with the result of the transformation
        std::mem::take(&mut self.manual_change)
            | match self.step_mode {
                StepMode::Immediate => {
                    self.rules.transform(&mut self.state);
                    self.last_step = Some(time::Instant::now());
                    true
                }
                StepMode::Limited { interval } => {
                    let step_permitted = self.last_step.unwrap().elapsed() >= interval;
                    if step_permitted {
                        self.rules.transform(&mut self.state);
                        self.last_step = Some(time::Instant::now());
                    }
                    step_permitted
                }
            }
    }

    /// Runs this automaton and displays it in a window.
    /// ```next_step()``` is called every frame, so setting an appropriate time step may be helpful for a smooth display.
    pub fn run_live(self) {
        pollster::block_on(crate::graphic::run_live(self));
    }
}
