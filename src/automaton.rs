use std::{collections::HashMap, time};

use crate::{error::CelluminaError, rule};

pub type CellGrid = grid::Grid<char>;

/// A struct that represents the current state and rule set of a cellular automaton.
pub struct Automaton {
    /// The current state of the automaton.
    pub(super) state: CellGrid,
    /// The rule set of the automaton.
    pub(super) rules: Box<dyn rule::Rule>,
    /// How often and on what conditions this automaton applies its rule set to its state to get to the next step.
    pub(super) step_mode: StepMode,
    /// The colors this automaton uses to convert itself to an image.
    pub(super) colors: HashMap<char, [u8; 4]>,
}

/// Describes how often an [Automaton] executes its time step.
pub(super) enum StepMode {
    /// Time steps are performed on every call of the [Automaton::next_step] function.
    Immediate,
    /// Time steps are performed every interval, but at most once per call of the [Automaton::next_step] function.
    Timed {
        interval: time::Duration,
        last_step: time::Instant,
    },
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
    /// The reason for this order is the column-major layouto of the underlying [grid::Grid] state representation.
    pub fn get_dimensions(&self) -> (u32, u32) {
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
            self.state[row as usize][col as usize] = new_val;
            Ok(())
        }
    }

    /// Checks wether the current [[StepMode]] permits a time step and, if it does, performs it.
    /// A time step consists of applying this automatons rule to its state.
    /// ## Returns
    /// Wether or not the next time step was
    pub fn next_step(&mut self) {
        match self.step_mode {
            StepMode::Immediate => {
                self.rules.transform(&mut self.state);
            }
            StepMode::Timed {
                interval,
                last_step,
            } => {
                if last_step.elapsed() >= interval {
                    self.rules.transform(&mut self.state);
                    self.step_mode = StepMode::Timed {
                        last_step: time::Instant::now(),
                        interval,
                    }
                }
            }
        }
    }
}
