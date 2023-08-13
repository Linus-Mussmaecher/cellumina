fn main() {
    cellumina::builder::AutomatonBuilder::new()
        .from_file("./tests/game_of_life/gol_init.txt")
        .with_rule(cellumina::rule::EnvironmentRule {
            range_vert: 1,
            range_hor: 1,
            cell_transform: |env| match env
                .iter()
                .enumerate()
                .map(|val| match val {
                    (4, 'X') => 0,
                    (_, 'X') => 1,
                    _ => 0,
                })
                .sum()
            {
                2 => env[1][1],
                3 => 'X',
                _ => ' ',
            },
        })
        .with_time_step(std::time::Duration::from_secs_f32(0.2))
        .with_color('X', [90, 201, 71, 255])
        .build()
        .run_live();
}
