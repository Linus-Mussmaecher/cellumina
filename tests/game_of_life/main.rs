/// This example implements John Conway's Game Of Life.
fn main() {
    // Create a new Cellular Automaton using the builder.
    cellumina::AutomatonBuilder::new()
        // Use an image to supply the initial configuration.
        .from_image_file("./tests/game_of_life/gol_init3.png")
        // Alternative source: Load the the initial state from a .txt file.
        //.from_text_file("./tests/game_of_life/gol_init2.txt")
        // Describe the rule of Conway's Game Of Life.
        .with_rule(cellumina::rule::EnvironmentRule {
            // Each cell only cares about neighbors 1 field away.
            range_vert: 1,
            range_hor: 1,
            cell_transform: |env| match env
            // Iterate over neighbors.
                .iter()
                .enumerate()
                .map(|val| match val {
                    // The cell we are transforming does not get counted.
                    (4, 'X') => 0,
                    // Any cell containing an 'X' counts for 1 (alive).
                    (_, 'X') => 1,
                    // Any cell containing any other entry (only ' ' in our initial configuration) counts as 0 (dead).
                    _ => 0,
                })
                // Sum over these 9 values...
                .sum()
                // ... and map the sum to the new enty of our cell:
            {
                // 2 neighbors: The cell keeps its state.
                2 => env[1][1],
                // 3 neighbors: The cell gets born.
                3 => 'X',
                // 0, 1 or more than 3 neighbors: The cell dies.
                _ => ' ',
            },
        })
        // Set a minimum time step.
        .with_min_time_step(std::time::Duration::from_secs_f32(0.2))
        // Set a display color for the live cells. This color needs to match the color of the live cells in our source image.
        .with_color('X', [95, 205, 228, 255])
        // Finish the build process.
        .build()
        // ... and run the automaton with graphical output.
        .run_live();
}
