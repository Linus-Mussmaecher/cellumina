fn main() {
    cellumina::AutomatonBuilder::new()
        .from_image_buffer(
            image::io::Reader::open("./tests/game_of_life/gol_init3.png")
                .expect("Could not read file.")
                .decode()
                .expect("Could not decode file.")
                .into_rgba8(),
        )
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
        .with_min_time_step(std::time::Duration::from_secs_f32(0.2))
        .with_color('X', [106, 190, 48, 255])
        .build()
        .run_live();
}
