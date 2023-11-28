/// This example implements a rock-paper-scissors cellular automaton.
fn main() {
    let size = 256 + 128;

    cellumina::AutomatonBuilder::new()
        // Generate a size x size initial state, with the top right, top left and bottom being the three colors
        .from_vec(
            (0..(size * size))
                .map(|index| {
                    if index > (size * size / 2) {
                        if index % size > size / 2 {
                            0
                        } else {
                            1
                        }
                    } else if index % size > size / 2 {
                        3
                    } else {
                        2
                    }
                })
                .collect(),
            size,
        )
        // set display colors
        // .with_color(0, [52, 75, 168, 255])
        // .with_color(1, [214, 185, 99, 255])
        // .with_color(2, [157, 165, 166, 255])
        // .with_color(3, [201, 120, 14, 255])
        .with_color(0, [66, 135, 245, 255])
        .with_color(1, [36, 80, 201, 255])
        .with_color(2, [61, 159, 235, 255])
        .with_color(3, [146, 199, 240, 255])
        // set progression rules
        .with_rule(cellumina::rule::EnvironmentRule {
            environment_size: [1, 1, 1, 1],
            // Periodic boundaries in both directions
            row_boundary: cellumina::rule::BoundaryBehaviour::Periodic,
            col_boundary: cellumina::rule::BoundaryBehaviour::Periodic,

            cell_transform: |grid| {
                let this = grid[1][1];
                let evil = (this + 1) % 4;
                let neutral = (this + 2) % 4;
                // if there are 3 or more cells that kill this one nearby -> kill this one with 70% chance
                if grid.iter().filter(|&&val| val == evil).count() >= 3
                    && rand::random::<f32>() < 0.7
                {
                    evil
                } else if grid.iter().filter(|&&val| val == neutral).count() >= 6
                // 6 more neutral cells so we can swallow grains
                    && rand::random::<f32>() < 0.7
                {
                    neutral
                } else {
                    this
                }
            },
        })
        // set time step
        .with_min_time_step(std::time::Duration::from_secs_f32(0.02))
        .build()
        .run_live();
}
