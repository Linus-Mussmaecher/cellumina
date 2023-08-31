use std::collections::HashMap;

use cellumina::rule::Pattern;

/// This example implements a falling-sand-simulation, and also features some other interactions.
fn main() {
    // Build an Automaton using the dedicated Builder struct.
    cellumina::AutomatonBuilder::new()
        // Use a text file as source of initial state.
        .from_text_file("./examples/sand/sand_init.txt")
        // Define how the automoton deals with the boundaries of the state grid.
        .with_pattern_edge_behaviour(
            cellumina::rule::BoundaryBehaviour::Symbol(126),
            cellumina::rule::BoundaryBehaviour::Symbol(126),
        )
        // Now specify the patters we want to use to transform our state.
        .with_patterns(&vec![
            // Sand (X or 59) falls down by one or even two spaces if possible.
            Pattern {
                before: grid::grid![[59][0][0]],
                after: grid::grid![[0][0][59]],
                priority: 1.0,
                chance: 0.9,
            },
            Pattern {
                before: grid::grid![[59][0]],
                after: grid::grid![[0][59]],
                priority: 0.5,
                ..Default::default()
            },
            // Stacks of sand collapse to the left or right. The shuffling of rules makes sure this does happen with equal probability.
            // Note the use of wildcards 127 in the 'after'-grids - these indicate to the automaton that the pattern does not mutate these cells.
            Pattern {
                before: grid::grid![[59, 0][59, 0]],
                after: grid::grid![[0, 127][127, 59]],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[0, 59][0, 59]],
                after: grid::grid![[127, 0][59, 127]],
                ..Default::default()
            },
            // Even 45 degree slopes of sand collapse (once again to both sides).
            Pattern {
                before: grid::grid![[59, 0, 0][59, 59, 0]],
                after: grid::grid![[0, 127, 127][127, 127, 59]],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[0, 0, 59][0, 59, 59]],
                after: grid::grid![[127, 127, 0][59, 127, 127]],
                ..Default::default()
            },
            // Fire has a small chance to fly upwards...
            Pattern {
                chance: 0.3,
                before: grid::grid![[0][41]],
                after: grid::grid![[41][0]],
                ..Default::default()
            },
            // ... and an even smaller chance to fall downwards.
            Pattern {
                chance: 0.1,
                before: grid::grid![[41][0]],
                after: grid::grid![[0][41]],
                ..Default::default()
            },
            // Also, fire can rarely move to the sides. All in all, this creates an upwards-trending random walk.
            Pattern {
                chance: 0.1,
                before: grid::grid![[0, 41]],
                after: grid::grid![[41, 0]],
                ..Default::default()
            },
            Pattern {
                chance: 0.1,
                before: grid::grid![[41, 0]],
                after: grid::grid![[0, 41]],
                ..Default::default()
            },
            // Fire above, below or next to sand ignites the sand.
            Pattern {
                chance: 0.8,
                before: grid::grid![[59][41]],
                after: grid::grid![[41][127]],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![[41][59]],
                after: grid::grid![[127][41]],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![[59, 41]],
                after: grid::grid![[41, 127]],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![[41, 59]],
                after: grid::grid![[127, 41]],
                ..Default::default()
            },
            // Fire can also ignite over corners - this requires another 4 rules.
            // This could also be solved by using an environment rule!
            // Note the use of the wildcard in the 'before' grid, as the pattern we are searching for is not rectangular and the contents of these cells do not matter to us.
            // The repeated wildcard pattern in the 'after' grid then ensures we also do not mutate these cells.
            Pattern {
                chance: 0.8,
                before: grid::grid![[59, 127][127, 41]],
                after: grid::grid![[41, 127][127, 127]],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![[127, 59][41, 127]],
                after: grid::grid![[127, 41][127, 127]],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![[127, 41][59, 127]],
                after: grid::grid![[127, 127][41, 127]],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![[41, 127][127, 59]],
                after: grid::grid![[127, 127][127, 41]],
                ..Default::default()
            },
            // Fire has a very small chance to decay to ash.
            Pattern {
                chance: 0.03,
                before: grid::grid![[41]],
                after: grid::grid![[36]],
                priority: 1.,
            },
            // Ash falls downwards at a slower pace than sand, no 2-move rule here.
            Pattern {
                before: grid::grid![[36][0]],
                after: grid::grid![[0][36]],
                ..Default::default()
            },
            // Just like sand, Ash collapses when stacked.
            Pattern {
                before: grid::grid![[36, 0][36, 0]],
                after: grid::grid![[0, 127][127, 36]],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[0, 36][0, 36]],
                after: grid::grid![[127, 0][36, 127]],
                ..Default::default()
            },
            // Fire does not ignite Ash, but passes cleanly through it and upwards
            Pattern {
                before: grid::grid![[36][41]],
                after: grid::grid![[41][36]],
                ..Default::default()
            },
            // Ash, just like fire, can ignite sand, but only from the 4 main directions.
            Pattern {
                before: grid::grid![[36][59]],
                after: grid::grid![[127][41]],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[59][36]],
                after: grid::grid![[41][127]],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[36, 59]],
                after: grid::grid![[127, 41]],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[59, 36]],
                after: grid::grid![[41, 127]],
                ..Default::default()
            },
            // Lastly, the Source has a 50% chance of spawning a fire cell above it every time step.
            Pattern {
                chance: 0.5,
                before: grid::grid![[127][54]],
                after: grid::grid![[41][54]],
                ..Default::default()
            },
        ])
        // Now set the colors the automaton uses for displaying these elements.
        .with_colors(HashMap::from([
            // space is nothing, so well use a soft blue as our background.
            (0, [61, 159, 184, 255]),
            // Sand
            (59, [224, 210, 159, 255]),
            // Fire
            (41, [224, 105, 54, 255]),
            // Ash
            (36, [184, 182, 182, 255]),
            // The Source
            (54, [128, 25, 14, 255]),
        ]))
        // Set a time step so the simulation runs at a consistent speed.
        .with_min_time_step(std::time::Duration::from_secs_f32(0.1))
        // Finish the build process.
        .build()
        // And use the Live View to run and display the automaton.
        .run_live();
}
