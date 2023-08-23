# Cellumina

[![Docs Status](https://docs.rs/mooeye/badge.svg)](https://docs.rs/cellumina)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Linus-Mussmaecher/cellumina/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/cellumina.svg)](https://crates.io/crates/cellumina)
[![Crates.io](https://img.shields.io/crates/d/cellumina.svg)](https://crates.io/crates/cellumina)

A library to easily crate and run [Cellular Automata](https://en.wikipedia.org/wiki/Cellular_automaton).

## Features

Cellumina provides an ```Automaton``` struct that represents a 2-dimensional grid of characters.
This grid can be initialized from a vector, a file or an image.
Additionally, the user can configure the ```Rule``` the automaton uses to transform itself into the next step.
The transformation to the next state can be initiated manually or on a fixed time step, for example when using Cellumina as part of a larger graphical application.

### Rules

* Pattern Replacement Rules
  * Specifiy a pattern that is searched each stepped and replaced with a second pattern.
  * Example: [Falling Sand Simulations](https://w-shadow.com/blog/2009/09/29/falling-sand-style-water-simulation/).
* Environment Rules
  * The next state of a cell is fully determined by its environment in the step before.
  * Example: [Rule 90](https://en.wikipedia.org/wiki/Rule_90).
  * Example: [Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

These rules can be added by creating these struct using normal Rust code.

The Patter Replacement Rules can also (de-)serialized by using ```serde``` or loaded from (and saved to) a custom file type.
This representation is more humanly readable than the serde version and can easily be created by hand if you do not want your rust files to contain large amounts of grid initializations for the patterns.

Additionally, the public trait [```Rule```](https://docs.rs/cellumina/latest/cellumina/rule/trait.Rule.html) can be overwritten to implement completely custom rules.

### Live View

Cellumina can be run in 'Live View' mode.
It will then take ownership of a configured automaton, run it by itself and display the cell state in a separate window.
This is useful when just playing around with cellular automata.

The user can also directly change the state of cells. Press any (character or space) button, and then mouse clicks will replace the currently hovered cell with the pressed character.
The simulation can also be paused and resumed with ```Enter```.

The live view functionality is not included in the library by default and must be enabled via the ```display``` feature.

#### Planned Features

The current state can be exported as a .txt or .png file.

## Usage

To use Cellumina in your own project, simply add this line to your ```Cargo.toml``` file:
```toml
  [dependencies]
  cellumina = "0.1"
```
or
```toml
  [dependencies]
  cellumina = {version = "0.1", features = ["display"]}
```
if you want to enable [live view](#live-view).

### Examples

The [examples folder](https://github.com/Linus-Mussmaecher/cellumina/tree/master/examples) contains the following examples:

 * ```game_of_life```: An implementation of conways game of life using environment rules.
 * ```sand```: A small falling sand simulation using pattern replacement rules to simulate falling sand, fire and ash.
 * ```to_string```: An example that shows how to convert rules to and from the different string/file types.

 All examples can be run by cloning this repository with
 ```bash
    git clone https://github.com/Linus-Mussmaecher/cellumina
 ```
 and executed by using ```cargo run --examples <name> --features="display"```, for example
  ```bash
    cargo run --examples sand --features="display"
  ```


### Performance

Since pattern replacement can be a rather costly operation, cellumina runs these in parallel using the [rayon](https://github.com/rayon-rs/rayon) crate.
Small patterns (as they may appear when e.g. using a falling sand simulation to create a death animation or similar) have negligible runtime.
Larger patters, especially with many patters or rules, may require more calculation time but can still be viewed in high FPS when running on their own.
Note that the runtime differs considerably between compilation in debug and release configuration.