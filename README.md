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

#### Planned Features

It will also be possible to load rules from files and save them.

### Live View

Cellumina can be run in 'Live View' mode.
It will then take ownership of a configured automaton, run it by itself and display the cell state in a separate window.
This is useful when just playing around with cellular automata.

The user can also directly change the state of cells. Press any (character or space) button, and then mouse clicks will replace the currently hovered cell with the pressed character.

#### Planned Features

 * Simulation control to allow the user to pause the simulation, speed it up or save the current state.

## Usage

To use Cellumina in your own project, simply add this line to your ```Cargo.toml``` file:
```toml
  [dependencies]
  cellumina = "0.1"
```

### Examples

The [tests folder](https://github.com/Linus-Mussmaecher/cellumina/tree/master/tests) contains example implementations of both Conway's Game Of Life and a small falling sand simulation. Clone the library and run ```cargo test``` to view them.

### Performance

Since pattern replacement can be a rather costly operation, cellumina runs these in parallel using the [rayon](https://github.com/rayon-rs/rayon) crate.
Small patterns (as they may appear when e.g. using a falling sand simulation to create a death animation or similar) have negligible runtime.
Larger patters, especially with many patters or rules, may require more calculation time but can still be viewed in high FPS when running on their own.
Note that the runtime considerably differs between compilation in debug and release configuration.