/// This example creates a labyrith from a starting point
fn main() {
    tree();
}

fn tree() {
    let size = 128;
    cellumina::AutomatonBuilder::new()
        // Generate a size x size initial state, all being nothing.
        // The user needs to set one cell to 1 to start the algo
        .from_vec(vec![0; size * size], size as u32)
        // set display colors
        .with_color(0, [88, 95, 107, 255]) // nothing
        .with_color(1, [50, 168, 131, 255]) // sources
        .with_color(2, [11, 82, 59, 255]) // main tree
        .with_color(3, [222, 199, 29, 255]) // lights
        .with_color(4, [72, 31, 7, 255]) // stump builder
        .with_color(5, [92, 51, 17, 255]) // branches
        .with_color(6, [82, 41, 7, 255]) // stump
        // set progression rules
        .with_rule(cellumina::rule::EnvironmentRule {
            environment_size: [1, 1, 1, 1],
            // Periodic boundaries in both directions
            row_boundary: cellumina::rule::BoundaryBehaviour::blocking_boundary(),
            col_boundary: cellumina::rule::BoundaryBehaviour::blocking_boundary(),

            cell_transform: |grid| {
                let this = grid[1][1];

                if this == 0
                // empty spaces below a stump builder
                    && (grid[0][1] == 4
                        // or, with 10% change left/right below a stump builder
                        || ((grid[0][0] == 4 && grid[1][0] == 0)
                            || (grid[0][2] == 4 && grid[1][2] == 0))
                            && rand::random::<f32>() < 0.1)
                {
                    // become stump builder
                    4
                } else if this == 4 && (grid[2][1] == 126 || grid[2][1] == 6 || grid[2][1] == 5) {
                    // stump buiders directly above the floow, a stump or a branch become a branch with 20% chance, if they are next to nothing else a stump
                    if grid[1][0] == 0 || grid[1][2] == 0 {
                        choose(0.1, 5, 6)
                    } else {
                        6
                    }
                } else if this == 0 && (grid[1][0] == 5 || grid[1][2] == 5) {
                    // empty spaces next to a branch become a branch with 90% chance but terminate as a source otherwise
                    choose(0.9, 5, 1)
                } else if this == 0
                    && (grid
                        .iter()
                        .skip(6)
                        .take(3)
                        .filter(|&&val| val == 1 || val == 2 || val == 5 || val == 3 || val == 6)
                        .count()
                        == 3)
                {
                    // empty spaces, where the three blocks below are all branches, lights, leaves or sources become a leaf, but a light with 5% chance
                    choose(0.95, 2, 3)
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

fn choose(chance: f32, def: u8, other: u8) -> u8 {
    if rand::random::<f32>() < chance {
        def
    } else {
        other
    }
}

#[allow(dead_code)]
fn labyrinth() {
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
