# Cellumina

[![Docs Status](https://docs.rs/mooeye/badge.svg)](https://docs.rs/cellumina)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Linus-Mussmaecher/cellumina/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/mooeye.svg)](https://crates.io/crates/cellumina)
[![Crates.io](https://img.shields.io/crates/d/mooeye.svg)](https://crates.io/crates/cellumina)

A library to easily crate and run [Cellular Automata](https://en.wikipedia.org/wiki/Cellular_automaton).

### Basic features

Cellumina provides an ```Automaton``` struct that represents a 2-dimensional grid of characters.
This grid can be initialized from a vector, a file or an image.
Additionally, the user can configure the ```Rule``` the automaton uses to transform itself into the next step.
The transformation to the next state can be initiated manually or on a fixed time step, for example when using Cellumina as part of a larger graphical application.

### Rules

* Pattern Replacement Rules
  * Specifiy a pattern that is searched each stepped and replaced with a second pattern.
  * Example: [Falling Sand Simulations](https://pvigier.github.io/2020/12/12/procedural-death-animation-with-falling-sand-automata.html).
* Environment Rules
  * The next state of a cell is fully determined by its environment in the step before.
  * Example: [Rule 90](https://en.wikipedia.org/wiki/Rule_90).
  * Example: [Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

### Live View

Cellumina can be run in 'Live View' mode.
It will then take ownership of a configured automaton, run it by itself and display the cell state in a separate window.
This is useful when just playing around with cellular automata.

### Performance

Since pattern replacement can be a rather costly operation, cellumina runs these in parallel using the [rayon](https://github.com/rayon-rs/rayon) crate.

### Planned Features

 * Interaction in Live View.
 * Loading rules from files and entire configurations from files.
 * Extensive examples.