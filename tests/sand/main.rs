use std::collections::HashMap;

use cellumina::rule::Pattern;

/// This example implements a falling-sand-simulation, and also features some other interactions.
fn main() {
    // Build an Automaton using the dedicated Builder struct.
    cellumina::AutomatonBuilder::new()
        // Use a text file as source of initial state.
        .from_text_file("./tests/sand/sand_init.txt")
        .with_pattern_edge_behaviour(cellumina::rule::EdgeBehaviour::Stop)
        // Now specify the patters we want to use to transform our state.
        .with_patterns(&vec![
            // Sand (X) falls down by one or even two spaces if possible.
            Pattern {
                before: grid::grid![['X'][' '][' ']],
                after: grid::grid![[' '][' ']['X']],
                priority: 1.0,
                chance: 0.9,
            },
            Pattern {
                before: grid::grid![['X'][' ']],
                after: grid::grid![[' ']['X']],
                priority: 0.5,
                ..Default::default()
            },
            // Stacks of sand collapse to the left or right. The shuffling of rules makes sure this does happen with equal probability.
            // Note the use of wildcards '*' in the 'after'-grids - these indicate to the automaton that the pattern does not mutate these cells.
            Pattern {
                before: grid::grid![['X', ' ']['X', ' ']],
                after: grid::grid![[' ', '*']['*', 'X']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[' ', 'X'][' ', 'X']],
                after: grid::grid![['*', ' ']['X', '*']],
                ..Default::default()
            },
            // Even 45 degree slopes of sand collapse (once again to both sides).
            Pattern {
                before: grid::grid![['X', ' ', ' ']['X', 'X', ' ']],
                after: grid::grid![[' ', '*', '*']['*', '*', 'X']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[' ', ' ', 'X'][' ', 'X', 'X']],
                after: grid::grid![['*', '*', ' ']['X', '*', '*']],
                ..Default::default()
            },
            // Fire has a small chance to fly upwards...
            Pattern {
                chance: 0.3,
                before: grid::grid![[' ']['F']],
                after: grid::grid![['F'][' ']],
                ..Default::default()
            },
            // ... and an even smaller chance to fall downwards.
            Pattern {
                chance: 0.1,
                before: grid::grid![['F'][' ']],
                after: grid::grid![[' ']['F']],
                ..Default::default()
            },
            // Also, fire can rarely move to the sides. All in all, this creates an upwards-trending random walk.
            Pattern {
                chance: 0.1,
                before: grid::grid![[' ', 'F']],
                after: grid::grid![['F', ' ']],
                ..Default::default()
            },
            Pattern {
                chance: 0.1,
                before: grid::grid![['F', ' ']],
                after: grid::grid![[' ', 'F']],
                ..Default::default()
            },
            // Fire above, below or next to sand ignites the sand.
            Pattern {
                chance: 0.8,
                before: grid::grid![['X']['F']],
                after: grid::grid![['F']['*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['F']['X']],
                after: grid::grid![['*']['F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['X', 'F']],
                after: grid::grid![['F', '*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['F', 'X']],
                after: grid::grid![['*', 'F']],
                ..Default::default()
            },
            // Fire can also ignite over corners - this requires another 4 rules.
            // This could also be solved by using an environment rule!
            // Note the use of the wildcard in the 'before' grid, as the pattern we are searching for is not rectangular and the contents of these cells do not matter to us.
            // The repeated wildcard pattern in the 'after' grid then ensures we also do not mutate these cells.
            Pattern {
                chance: 0.8,
                before: grid::grid![['X', '*']['*', 'F']],
                after: grid::grid![['F', '*']['*', '*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['*', 'X']['F', '*']],
                after: grid::grid![['*', 'F']['*', '*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['*', 'F']['X', '*']],
                after: grid::grid![['*', '*']['F', '*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['F', '*']['*', 'X']],
                after: grid::grid![['*', '*']['*', 'F']],
                ..Default::default()
            },
            // Fire has a very small chance to decay to ash.
            Pattern {
                chance: 0.03,
                before: grid::grid![['F']],
                after: grid::grid![['A']],
                priority: 1.,
            },
            // Ash falls downwards at a slower pace than sand, no 2-move rule here.
            Pattern {
                before: grid::grid![['A'][' ']],
                after: grid::grid![[' ']['A']],
                ..Default::default()
            },
            // Just like sand, Ash collapses when stacked.
            Pattern {
                before: grid::grid![['A', ' ']['A', ' ']],
                after: grid::grid![[' ', '*']['*', 'A']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[' ', 'A'][' ', 'A']],
                after: grid::grid![['*', ' ']['A', '*']],
                ..Default::default()
            },
            // Fire does not ignite Ash, but passes cleanly through it and upwards
            Pattern {
                before: grid::grid![['A']['F']],
                after: grid::grid![['F']['A']],
                ..Default::default()
            },
            // Ash, just like fire, can ignite sand, but only from the 4 main directions.
            Pattern {
                before: grid::grid![['A']['X']],
                after: grid::grid![['*']['F']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['X']['A']],
                after: grid::grid![['F']['*']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['A', 'X']],
                after: grid::grid![['*', 'F']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['X', 'A']],
                after: grid::grid![['F', '*']],
                ..Default::default()
            },
            // Lastly, the Source has a 50% chance of spawning a fire cell above it every time step.
            Pattern {
                chance: 0.5,
                before: grid::grid![['*']['S']],
                after: grid::grid![['F']['S']],
                ..Default::default()
            },
        ])
        // Now set the colors the automaton uses for displaying these elements.
        .with_colors(HashMap::from([
            // space is nothing, so well use a soft blue as our background.
            (' ', [61, 159, 184, 255]),
            // Sand
            ('X', [224, 210, 159, 255]),
            // Fire
            ('F', [224, 105, 54, 255]),
            // Ash
            ('A', [184, 182, 182, 255]),
            // The Source
            ('S', [128, 25, 14, 255]),
        ]))
        // Set a time step so the simulation runs at a consistent speed.
        .with_min_time_step(std::time::Duration::from_secs_f32(0.1))
        // Finish the build process.
        .build()
        // And use the Live View to run and display the automaton.
        .run_live();
}
