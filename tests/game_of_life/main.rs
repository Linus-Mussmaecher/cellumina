fn main() {
    pollster::block_on(cellumina::graphic::state::run(
        "./tests/game_of_life/gol_init.txt",
    ));
}
