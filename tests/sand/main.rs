use std::collections::HashMap;

use cellumina::rule::Pattern;

fn main() {
    cellumina::AutomatonBuilder::new()
        .from_text_file("./tests/sand/sand_init.txt")
        .with_patterns(&vec![
            Pattern {
                before: grid::grid![['X'][' '][' ']],
                after: grid::grid![[' '][' ']['X']],
                priority: 1.0,
                chance: 0.9,
            },
            Pattern {
                before: grid::grid![['X'][' ']],
                after: grid::grid![[' ']['X']],
                priority: 0.5,
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['X', ' ']['X', ' ']],
                after: grid::grid![[' ', ' ']['X', 'X']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[' ', 'X'][' ', 'X']],
                after: grid::grid![[' ', ' ']['X', 'X']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['X', ' ', ' ']['X', 'X', ' ']],
                after: grid::grid![[' ', ' ', ' ']['X', 'X', 'X']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[' ', ' ', 'X'][' ', 'X', 'X']],
                after: grid::grid![[' ', ' ', ' ']['X', 'X', 'X']],
                ..Default::default()
            },
            Pattern {
                chance: 0.3,
                before: grid::grid![[' ']['F']],
                after: grid::grid![['F'][' ']],
                ..Default::default()
            },
            Pattern {
                chance: 0.1,
                before: grid::grid![['F'][' ']],
                after: grid::grid![[' ']['F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['X']['F']],
                after: grid::grid![['F']['F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['X', 'F']],
                after: grid::grid![['F', 'F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['F', 'X']],
                after: grid::grid![['F', 'F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['F']['X']],
                after: grid::grid![['F']['F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['X', '*']['*', 'F']],
                after: grid::grid![['F', '*']['*', '*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['*', 'X']['F', '*']],
                after: grid::grid![['*', 'F']['*', '*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['*', 'F']['X', '*']],
                after: grid::grid![['*', '*']['F', '*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.8,
                before: grid::grid![['F', '*']['*', 'X']],
                after: grid::grid![['*', '*']['*', 'F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.03,
                before: grid::grid![['F']],
                after: grid::grid![['A']],
                priority: 1.,
            },
            Pattern {
                before: grid::grid![['A'][' ']],
                after: grid::grid![[' ']['A']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['A', ' ']['A', ' ']],
                after: grid::grid![[' ', '*']['A', 'A']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![[' ', 'A'][' ', 'A']],
                after: grid::grid![['*', ' ']['A', 'A']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['A']['F']],
                after: grid::grid![['F']['A']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['A']['X']],
                after: grid::grid![[' ']['F']],
                ..Default::default()
            },
            Pattern {
                before: grid::grid![['X']['A']],
                after: grid::grid![['F']['*']],
                ..Default::default()
            },
            Pattern {
                chance: 0.1,
                before: grid::grid![[' ', 'F']],
                after: grid::grid![['F', ' ']],
                ..Default::default()
            },
            Pattern {
                chance: 0.1,
                before: grid::grid![['F', ' ']],
                after: grid::grid![[' ', 'F']],
                ..Default::default()
            },
            Pattern {
                chance: 0.5,
                before: grid::grid![['*']['S']],
                after: grid::grid![['F']['S']],
                ..Default::default()
            },
        ])
        .with_colors(HashMap::from([
            (' ', [0; 4]),
            ('X', [86, 181, 78, 255]), //[232, 212, 100, 255],
            ('F', [235, 64, 52, 255]),
            ('A', [235, 125, 125, 255]),
            ('S', [185, 23, 45, 255]),
        ]))
        .with_min_time_step(std::time::Duration::from_secs_f32(0.1))
        .build()
        .run_live();
}
