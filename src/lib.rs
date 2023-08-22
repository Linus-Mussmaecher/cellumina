mod automaton;
pub use automaton::Automaton;

mod builder;
pub use builder::AutomatonBuilder;

mod error;
pub use error::CelluminaError;

/// Contains the model, view and controller for diplaying automata.
pub(crate) mod graphic;
/// Contains structs and traits for the definition of the transformations rules of cellular automata.
pub mod rule;

/// A type for the underlying state of a cellular automaton.
/// Each cell always has a character as a state in cellumina.
pub type CellGrid = grid::Grid<char>;
