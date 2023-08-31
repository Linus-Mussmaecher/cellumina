/// This example implements John Conway's Game Of Life.
fn main() {
    // Create a new Cellular Automaton using the builder.
    cellumina::AutomatonBuilder::new()
        // Use an image to supply the initial configuration.
        .from_image_file("./examples/game_of_life/gol_init3.png")
        // This is a considerably larger image. Running this on debug mode might be slow.
        //.from_image_file("./examples/game_of_life/gol_init3.png")
        // Alternative source: Load the the initial state from a .txt file.
        //.from_text_file("./examples/game_of_life/gol_init2.txt")
        // Describe the rule of Conway's Game Of Life.
        .with_rule(cellumina::rule::EnvironmentRule {
            // Each cell only cares about neighbors 1 field away, in every direction.
            environment_size: [1, 1, 1, 1],
            row_boundary: cellumina::rule::BoundaryBehaviour::Symbol(0),
            col_boundary: cellumina::rule::BoundaryBehaviour::Symbol(0),
            cell_transform: |env| match env
                // Iterate over neighbors.
                .iter().copied()
                // Sum over these 9 values without the center
                .sum::<u8>() - env[1][1]
                // ... and map the sum to the new enty of our cell:
            {
                // 2 neighbors: The cell keeps its state.
                2 => env[1][1],
                // 3 neighbors: The cell gets born.
                3 => 1,
                // 0, 1 or more than 3 neighbors: The cell dies.
                _ => 0,
            },
        })
        // Set a minimum time step.
        .with_min_time_step(std::time::Duration::from_secs_f32(0.1))
        // Set a display color for the live cells. This color needs to match the color of the live cells in our source image.
        .with_color(1, [95, 205, 228, 255])
        // Finish the build process.
        .build()
        // And run the automaton with graphical output.
        .run_live();
}
