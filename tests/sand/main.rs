fn main() {
    pollster::block_on(cellumina::graphic::state::run("./tests/sand/sand_init.txt"));
}
