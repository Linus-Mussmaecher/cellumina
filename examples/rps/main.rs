/// This example implements the famous rule90 cellular automaton.
fn main() {
    let size = 256;

    cellumina::AutomatonBuilder::new()
        // Generate a 64x64 initial state, with the first row having random values and the rest being empty
        // Each row will represent one time step of the automaton.
        .from_vec(
            (0..(size * size))
                .map(|index| {
                    if index > (size * size / 2) {
                        0
                    } else if index % size > size / 2 {
                        1
                    } else {
                        2
                    }
                })
                .collect(),
            size,
        )
        // set display colors
        .with_color(0, [52, 75, 168, 255])
        .with_color(1, [214, 185, 99, 255])
        .with_color(2, [157, 165, 166, 255])
        // set progression rules
        .with_rule(cellumina::rule::EnvironmentRule {
            environment_size: [1, 1, 1, 1],
            // True boundaries in both directions
            row_boundary: cellumina::rule::BoundaryBehaviour::Periodic,
            col_boundary: cellumina::rule::BoundaryBehaviour::Periodic,

            cell_transform: |grid| {
                let this = grid[1][1];
                let evil = (this + 1) % 3;
                if grid.iter().filter(|&&val| val == evil).count() >= 3
                    && rand::random::<f32>() < 0.4
                {
                    evil
                } else {
                    this
                }
            },
        })
        // set time step
        .with_min_time_step(std::time::Duration::from_secs_f32(0.1))
        .build()
        .run_live();
}
