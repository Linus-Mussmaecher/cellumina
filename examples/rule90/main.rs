/// This example implements the famous rule90 cellular automaton.
fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    cellumina::AutomatonBuilder::new()
        // Generate a 64x64 initial state, with the first row having random values and the rest being empty
        // Each row will represent one time step of the automaton.
        .from_vec(
            (0..4096)
                .map(|index| {
                    if index <= 64 * 63 || rand::random::<u8>() % 2 == 0 {
                        '0'
                    } else {
                        '1'
                    }
                })
                .collect(),
            64,
        )
        // set display colors: 1 is displayed as white, 0 as black
        .with_color('1', [255, 255, 255, 255])
        .with_color('0', [0, 0, 0, 255])
        // set progression rules
        .with_rule(cellumina::rule::EnvironmentRule {
            // we need to look one row up (to the last state)
            row_range: 1,
            // and one row to each side
            col_range: 1,
            boundaries: (
                // Towards the top and bottom, we have a true boundary.
                cellumina::rule::BoundaryBehaviour::Symbol('_'),
                // Towards the left and right, we pretend there are always zeroes.
                cellumina::rule::BoundaryBehaviour::Symbol('0'),
            ),
            cell_transform: |grid| {
                // Top row (marked by the row above it containing only '_', the out-of-bounds-symbol) eternally keeps its value.
                if grid[2][1] == '_' {
                    if (grid[1][0] == '1') ^ (grid[1][2] == '1') {
                        '1'
                    } else {
                        '0'
                    }
                }
                // Other rows generate the value as the exclusive or of the two neighbors last time step (one row above).
                else {
                    grid[2][1]
                }
            },
        })
        // set time step
        .with_min_time_step(std::time::Duration::from_secs_f32(0.2))
        .build()
        .run_live();
}
