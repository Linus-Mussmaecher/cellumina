/// This example creates a labyrith from a starting point
fn main() {
    let size = 128;

    cellumina::AutomatonBuilder::new()
        // Generate a size x size initial state, all being nothing.
        // The user needs to set one cell to 1 to start the algo
        .from_vec(vec![0; size * size], size as u32)
        // set display colors
        .with_color(0, [88, 95, 107, 255]) // nothing
        .with_color(1, [220, 223, 227, 255]) // hallway
        .with_color(2, [88, 95, 107, 255]) // wall
        .with_color(3, [88, 95, 107, 255]) // wall
        // set progression rules
        .with_rule(cellumina::rule::EnvironmentRule {
            environment_size: [1, 1, 1, 1],
            // Periodic boundaries in both directions
            row_boundary: cellumina::rule::BoundaryBehaviour::Periodic,
            col_boundary: cellumina::rule::BoundaryBehaviour::Periodic,

            cell_transform: |grid| {
                let this = grid[1][1];

                if this == 0 {
                    //exactly one horizontal neighbor
                    if grid[0][1] + grid[2][1] + grid[1][0] + grid[1][2] == 1
                        && rand::random::<f32>() < 0.3
                    {
                        1
                    } else {
                        match grid.iter().filter(|&&val| val == 1).count() {
                            2 => {
                                if rand::random::<f32>() < 0.2 {
                                    1
                                } else if rand::random::<f32>() < 0.85 {
                                    2
                                } else {
                                    0
                                }
                            }
                            3 => {
                                if rand::random::<f32>() < 0.4 {
                                    1
                                } else {
                                    0
                                }
                            }
                            4 => {
                                if rand::random::<f32>() < 0.6 {
                                    1
                                } else {
                                    0
                                }
                            }
                            5 | 6 | 7 | 8 | 9 => 1,
                            _ => 0,
                        }
                    }
                } else if this == 2 {
                    // clear 'pillars'
                    match grid.iter().filter(|&&val| val == 1).count() {
                        6 | 7 | 8 | 9 => 1,
                        _ => 2,
                    }
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
