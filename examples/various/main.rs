/// This example creates a labyrith from a starting point
fn main() {
    tree();
}

fn tree() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let size = 128;
    cellumina::AutomatonBuilder::new()
        // Generate a size x size initial state, all being nothing.
        // The user needs to set one cell to 1 to start the algo
        .from_vec(vec![0; size * size], size as u32)
        // set display colors
        .with_color(0, [88, 95, 107, 255]) // nothing
        .with_color(1, [11, 82, 59, 255]) // main tree
        .with_color(2, [222, 199, 29, 255]) // lights
        .with_color(3, [72, 31, 7, 255]) // stump builder
        .with_color(4, [16, 87, 64, 255]) // branch source
        .with_color(5, [88, 95, 107, 255]) // branch
        //.with_color(5, [93, 100, 112, 255]) // branches
        .with_color(6, [82, 41, 7, 255]) // stump remains
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
                    && (grid[0][1] == 3
                        // or, with 10% change left/right below a stump builder
                        || ((grid[0][0] == 3 && grid[1][0] == 0)
                            || (grid[0][2] == 3 && grid[1][2] == 0))
                            && rand::random::<f32>() < 0.4)
                {
                    // become stump builder
                    3
                } else if this == 3 && (grid[2][1] != 0 && grid[2][1] != 3) {
                    // stump buiders directly above non-empty, non-stump-builder next to nothing become a branch source with small chance,
                    // else just stump remains
                    // top one guaranteed to be branch source
                    if (grid[0][0] == 0 && grid[0][1] == 0 && grid[0][2] == 0)
                        || ((grid[1][0] == 0 || grid[1][2] == 0) && rand::random::<f32>() < 0.12)
                    {
                        4
                    } else {
                        6
                    }
                } else if this == 0
                    && (grid[1][0] == 5 || grid[1][2] == 5 || grid[1][0] == 4 || grid[1][2] == 4)
                {
                    // empty spaces next to a branch (-source) become a branch
                    5
                } else if (this == 0 || this == 6)
                    && (grid
                        .iter()
                        .take(3)
                        .filter(|&&val| val == 4 || val == 1 || val == 2))
                    .count()
                        > 0
                {
                    choose(0.95, 1, 2)
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
