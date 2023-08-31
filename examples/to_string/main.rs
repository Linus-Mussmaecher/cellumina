/// Tests the conversion of Pattern and PatternRule to and from String.
fn main() {
    // Let's create pretty basic rule.
    let rule = cellumina::rule::PatternRule::from_patterns(
        &[
            cellumina::rule::Pattern {
                chance: 1.0,
                priority: 1.0,
                before: grid::grid![[1][0]],
                after: grid::grid![[0][1]],
            },
            cellumina::rule::Pattern {
                chance: 0.8,
                priority: 0.5,
                before: grid::grid![[1, 0][1, 0]],
                after: grid::grid![[0, 0][1, 1]],
            },
            cellumina::rule::Pattern {
                chance: 0.8,
                priority: 0.5,
                before: grid::grid![[0, 1][0, 1]],
                after: grid::grid![[0, 0][1, 1]],
            },
        ],
        cellumina::rule::BoundaryBehaviour::Periodic,
        cellumina::rule::BoundaryBehaviour::Periodic,
    );

    // Rules can be converted to strings.
    // The rule can be recreated from the display output.
    let _rule2 = cellumina::rule::PatternRule::from(rule.to_string().as_str());

    // Therefore, you can save this output to a file and reload the rule later (or even type such a file yourself so the rule and patterns do not need to be created in code.)

    let path = "./examples/to_string/basic_rule.cel";

    std::fs::write(path, rule.to_string()).expect("Could not write to file!");

    // Can also be converted to a file via serde:

    std::fs::write(
        "./examples/to_string/basic_rule_serde.toml",
        toml::to_string(&rule).expect("Could not convert to TOML string."),
    )
    .expect("Could not write to file!");

    let _rule3 = cellumina::rule::PatternRule::from(
        std::fs::read_to_string(path)
            .expect("Could not read file!")
            .as_str(),
    );

    // The sand_rules.cel file contains an exported copy of the sand rules from example 'sand'. Let's load it and run it.
    cellumina::AutomatonBuilder::new()
        // Load the rule from the file.
        .with_rule(cellumina::rule::PatternRule::from(
            std::fs::read_to_string("./examples/to_string/sand_rules.cel")
                .expect("Could not read file.")
                .as_str(),
        ))
        // The initial state is loaded from a different file.
        .from_text_file("./examples/sand/sand_init.txt")
        // Set the colors again
        .with_colors(std::collections::HashMap::from([
            // L is nothing, so well use a soft blue as our background.
            (cellumina::char_to_id(' '), [61, 159, 184, 255]),
            // Sand
            (cellumina::char_to_id('X'), [224, 210, 159, 255]),
            // Fire
            (cellumina::char_to_id('F'), [224, 105, 54, 255]),
            // Ash
            (cellumina::char_to_id('A'), [184, 182, 182, 255]),
            // The Source
            (cellumina::char_to_id('S'), [128, 25, 14, 255]),
        ]))
        // Set a time step.
        .with_min_time_step(std::time::Duration::from_secs_f32(0.1))
        .build()
        .run_live();
}
