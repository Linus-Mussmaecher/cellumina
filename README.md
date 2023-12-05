# Cellumina

[![Docs Status](https://docs.rs/mooeye/badge.svg)](https://docs.rs/cellumina)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Linus-Mussmaecher/cellumina/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/cellumina.svg)](https://crates.io/crates/cellumina)
[![Crates.io](https://img.shields.io/crates/d/cellumina.svg)](https://crates.io/crates/cellumina)

A library to easily create and run [Cellular Automata](https://en.wikipedia.org/wiki/Cellular_automaton).

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
If you want to clear the whole screen and fill every cell with the same character, press ```Alt``` and that character.
This works with all alphanumeric characters, but is currently not supported for space - you'll have to use ```Alt + 0``` instead.

The automaton can also be paused and resumed with ```Enter```.
The current state of the automaton can be saved to a file with ```Ctrl + S```, currently the following formats are supported: ```txt``` (with one row of chararcters per line) as well as ```png, jpeg, ico, bmp```. Normal restrictions of those files apply, e.g. saving to jpeg may result in compression, so ```.jpeg```-files are not suited for saving and reloading automata.

The live view functionality is not included in the library by default and must be enabled via the ```display``` feature.

## Usage

To use Cellumina in your own project, simply add this line to your ```Cargo.toml``` file:
```toml
  [dependencies]
  cellumina = "0.2"
```
or
```toml
  [dependencies]
  cellumina = {version = "0.2", features = ["display"]}
```
if you want to enable [live view](#live-view).

### Examples

The [examples folder](https://github.com/Linus-Mussmaecher/cellumina/tree/master/examples) contains the following examples:

 * ```game_of_life```: An implementation of conways game of life using environment rules.
 * ```sand```: A small falling sand simulation using pattern replacement rules to simulate falling sand, fire and ash.
 * ```rule90```: A implementation of the [Rule 90](https://en.wikipedia.org/wiki/Rule_90) 1-dimensional cellular automaton that demonstrates how to use Cellumina's 2D-grid to display multiple successive states of a 1-dimensional automaton.
 * ```to_string```: An example that shows how to convert rules to and from the different string/file types.
 * ```rps```: An environment-based system of four different cell states that circularly annihilate each other (as in rock-paper-scissors), creating pleasing wave patterns.
 * ```various```: Various different automata that create a finished, static state from a set of rules, such as a labyrith pattern or a christmas tree.

 All examples can be run by cloning this repository with
 ```bash
    git clone https://github.com/Linus-Mussmaecher/cellumina
 ```
 and executed by using ```cargo run --examples <name> --features="display"```, for example
  ```bash
    cargo run --examples sand --features="display"
  ```
 the ```to_string``` example additionaly requires the ```simple_logger``` feature to demonstrate logging.

### Logging

Cellumina supports logging via the [log](https://docs.rs/log/latest/log/) crate.
You can use any logger, such as [env-log](https://docs.rs/env_logger/latest/env_logger/) or [simple-logger](https://docs.rs/simple_logger/latest/simple_logger/), initialize them as described in their documentations and receive log outputs from cellumina.

### Performance

Since pattern replacement can be a rather costly operation, cellumina runs these in parallel using the [rayon](https://github.com/rayon-rs/rayon) crate.
Small patterns (as they may appear when e.g. using a falling sand simulation to create a death animation or similar) have negligible runtime.
Larger patters, especially with many patters or rules, may require more calculation time but can still be viewed in high FPS when running on their own.
Note that the runtime differs considerably between compilation in debug and release configuration.