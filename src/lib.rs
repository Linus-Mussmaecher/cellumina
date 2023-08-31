//! [![Docs Status](https://docs.rs/mooeye/badge.svg)](https://docs.rs/cellumina)
//! [![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Linus-Mussmaecher/cellumina/blob/main/LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/cellumina.svg)](https://crates.io/crates/cellumina)
//! [![Crates.io](https://img.shields.io/crates/d/cellumina.svg)](https://crates.io/crates/cellumina)
//!
//! A library to easily create and run [Cellular Automata](https://en.wikipedia.org/wiki/Cellular_automaton).
//!
//! ## Features
//!
//! Cellumina provides an ```Automaton``` struct that represents a 2-dimensional grid of characters.
//! This grid can be initialized from a vector, a file or an image.
//! Additionally, the user can configure the ```Rule``` the automaton uses to transform itself into the next step.
//! The transformation to the next state can be initiated manually or on a fixed time step, for example when using Cellumina as part of a larger graphical application.
//!
//! ### Rules
//!
//! * Pattern Replacement Rules
//!   * Specifiy a pattern that is searched each stepped and replaced with a second pattern.
//!   * Example: [Falling Sand Simulations](https://w-shadow.com/blog/2009/09/29/falling-sand-style-water-simulation/).
//! * Environment Rules
//!   * The next state of a cell is fully determined by its environment in the step before.
//!   * Example: [Rule 90](https://en.wikipedia.org/wiki/Rule_90).
//!   * Example: [Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).
//!
//! These rules can be added by creating these struct using normal Rust code.
//!
//! The Patter Replacement Rules can also (de-)serialized by using ```serde``` or loaded from (and saved to) a custom file type.
//! This representation is more humanly readable than the serde version and can easily be created by hand if you do not want your rust files to contain large amounts of grid initializations for the patterns.
//!
//! Additionally, the public trait [```Rule```](https://docs.rs/cellumina/latest/cellumina/rule/trait.Rule.html) can be overwritten to implement completely custom rules.
//!
//! ### Live View
//!
//! Cellumina can be run in 'Live View' mode.
//! It will then take ownership of a configured automaton, run it by itself and display the cell state in a separate window.
//! This is useful when just playing around with cellular automata.
//!
//! The user can also directly change the state of cells. Press any (character or space) button, and then mouse clicks will replace the currently hovered cell with the pressed character.
//! The automaton can also be paused and resumed with ```Enter```.
//! The current state of the automaton can be saved to a file with ```Ctrl + S```, currently the following formats are supported: ```txt``` (with one row of chararcters per line) as well as ```png, jpeg, ico, bmp```. Normal restrictions of those files apply, e.g. saving to jpeg may result in compression, so ```.jpeg```-files are not suited for saving and reloading automata.
//!
//! The live view functionality is not included in the library by default and must be enabled via the ```display``` feature.
//!
//! ## Usage
//!
//! To use Cellumina in your own project, simply add this line to your ```Cargo.toml``` file:
//! ```toml
//!   [dependencies]
//!   cellumina = "0.2"
//! ```
//! or
//! ```toml
//!   [dependencies]
//!   cellumina = {version = "0.2", features = ["display"]}
//! ```
//! if you want to enable [live view](#live-view).
//!
//! ### Examples
//!
//! The [examples folder](https://github.com/Linus-Mussmaecher/cellumina/tree/master/examples) contains the following examples:
//!
//! * ```game_of_life```: An implementation of conways game of life using environment rules.
//! * ```sand```: A small falling sand simulation using pattern replacement rules to simulate falling sand, fire and ash.
//! * ```rule90```: A implementation of the [Rule 90](https://en.wikipedia.org/wiki/Rule_90) 1-dimensional cellular automaton that demonstrates how to use Cellumina's 2D-grid to display multiple successive states of a 1-dimensional automaton.
//! * ```to_string```: An example that shows how to convert rules to and from the different string/file types.
//!
//!  All examples can be run by cloning this repository with
//!  ```bash
//!     git clone https://github.com/Linus-Mussmaecher/cellumina
//!  ```
//!  and executed by using ```cargo run --examples <name> --features="display"```, for example
//!   ```bash
//!     cargo run --examples sand --features="display"
//!   ```
//!
//! ### Logging
//!
//! Cellumina supports logging via the [log](https://docs.rs/log/latest/log/) crate.
//! You can use any logger, such as [env-log](https://docs.rs/env_logger/latest/env_logger/) or [simple-logger](https://docs.rs/simple_logger/latest/simple_logger/), initialize them as described in their documentations and receive log outputs from cellumina.
//!
//! ### Performance
//!
//! Since pattern replacement can be a rather costly operation, cellumina runs these in parallel using the [rayon](https://github.com/rayon-rs/rayon) crate.
//! Small patterns (as they may appear when e.g. using a falling sand simulation to create a death animation or similar) have negligible runtime.
//! Larger patters, especially with many patters or rules, may require more calculation time but can still be viewed in high FPS when running on their own.
//! Note that the runtime differs considerably between compilation in debug and release configuration.

mod automaton;
pub use automaton::Automaton;

mod builder;
pub use builder::AutomatonBuilder;

mod error;
pub use error::CelluminaError;

/// Contains the model, view and controller for diplaying automata.
#[cfg(feature = "display")]
pub(crate) mod graphic;
/// Contains structs and traits for the definition of the transformations rules of cellular automata.
pub mod rule;

/// A type for the underlying state of a cellular automaton.
/// Each cell always has a character as a state in cellumina.
pub type CellGrid = grid::Grid<u8>;

/// Converts each character to its associated u8 value.
///
/// ```
///     # use cellumina::char_to_id;
///     assert_eq!(char_to_id('0'), 0);
///     assert_eq!(char_to_id('5'), 5);
///     assert_eq!(char_to_id('b'), 11);
///     assert_eq!(char_to_id('A'), 36);
/// ```
/// PREFERABLY RESERVED:
///     0 - Used for space
///     126 - Default border symbol
/// CERTAINLY RESERVED:
///     127 - Pattern Rule wildcard
/// ```
///     # use cellumina::char_to_id;
///     assert_eq!(char_to_id(' '), 0);
///     assert_eq!(char_to_id('*'), 127);
///     assert_eq!(char_to_id('_'), 126);
/// ```
pub const fn char_to_id(symbol: char) -> u8 {
    match symbol {
        '0'..='9' => (symbol as u32 - 48) as u8,
        'a'..='z' => (symbol as u32 - 97 + 10) as u8,
        'A'..='Z' => (symbol as u32 - 65 + 10 + 26) as u8,
        '_' => 126,
        '*' => 127,
        ' ' => 0,
        _ => 0,
    }
}

/// Converts an u8 value to its associated character.
/// ```
///     # use cellumina::id_to_char;
///     assert_eq!(id_to_char(0), ' ');
///     assert_eq!(id_to_char(5), '5');
///     assert_eq!(id_to_char(11), 'b');
///     assert_eq!(id_to_char(36), 'A');
/// ```
///
/// PREFERABLY RESERVED:
///     0 - Used for space
///     126 - Default border symbol _
/// CERTAINLY RESERVED:
///     127 - Pattern Rule wildcard *
/// ```
///     # use cellumina::id_to_char;
///     assert_eq!(id_to_char(0), ' ');
///     assert_ne!(id_to_char(0), '0');
///     assert_eq!(id_to_char(127), '*');
///     assert_eq!(id_to_char(126), '_');
/// ```
pub const fn id_to_char(id: u8) -> char {
    match id {
        0 => ' ',
        1..=9 => (id + 48) as char,
        10..=35 => (id + 97 - 10) as char,
        36..=63 => (id + 65 - 10 - 26) as char,
        126 => '_',
        127 => '*',
        _ => ' ',
    }
}

#[test]
fn conversion_test() {
    assert_eq!(
        ('0'..='9').map(char_to_id).collect::<Vec<u8>>(),
        (0..10).collect::<Vec<u8>>()
    );
    assert_eq!(
        ('a'..='z').map(char_to_id).collect::<Vec<u8>>(),
        (10..36).collect::<Vec<u8>>()
    );
    assert_eq!(
        ('A'..='Z').map(char_to_id).collect::<Vec<u8>>(),
        (36..62).collect::<Vec<u8>>()
    );
    assert_eq!(
        (1..10).map(id_to_char).collect::<Vec<char>>(),
        ('1'..='9').collect::<Vec<char>>()
    );
    assert_eq!(
        (10..36).map(id_to_char).collect::<Vec<char>>(),
        ('a'..='z').collect::<Vec<char>>()
    );
    assert_eq!(
        (36..62).map(id_to_char).collect::<Vec<char>>(),
        ('A'..='Z').collect::<Vec<char>>()
    );
}
