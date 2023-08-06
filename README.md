# Cellumina

A WebGL-based program to run cellular automata via the command line. Planned features:

### Basic features

Cellumina is a Command Line Programm to run 1- or 2-dimensional cellular automata.
Cellumina loads an initial state from the target folder, displays it in a rendering window and repeatedly modifies it by applying a fully customizable ruleset to the cells.

The colouring of the display, the set of rules loaded and the default timestep can be freely configured via a config file.

### Runtime Interaction
* Manipulate cells at runtime
* Pause, speed up or slow down interaction
* Save the current state.

### Rules

Fully customize the rules of the cellular automaton running in cellumina by selectively loading rules from module files.
Possible rule types include:

* Pattern Replacement Rules
  * Specifiy a pattern that is searched each stepped and replaced with a second pattern.
  * Example: [Falling Sand Simulations](https://pvigier.github.io/2020/12/12/procedural-death-animation-with-falling-sand-automata.html).
* Environment Rules
  * The next state of a cell is fully determined by its environment in the step before.
  * Example: [Rule 90](https://en.wikipedia.org/wiki/Rule_90).
* Counting Rules
  * A variation of environment rules that only relies on the count of surrounding elements.
  * Example: [Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

### Performance
* Multithreading with rayon OR
* Compute shaders
