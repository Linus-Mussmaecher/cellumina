/// Tests the conversion of Pattern and PatternRule to and from String.
fn main() {
    // Let's create pretty basic rule.
    let rule = cellumina::rule::PatternRule::from_patterns(
        &[
            cellumina::rule::Pattern {
                chance: 1.0,
                priority: 1.0,
                before: grid::grid![['X'][' ']],
                after: grid::grid![[' ']['X']],
            },
            cellumina::rule::Pattern {
                chance: 0.8,
                priority: 0.5,
                before: grid::grid![['X', ' ']['X', ' ']],
                after: grid::grid![[' ', ' ']['X', 'X']],
            },
            cellumina::rule::Pattern {
                chance: 0.8,
                priority: 0.5,
                before: grid::grid![[' ', 'X'][' ', 'X']],
                after: grid::grid![[' ', ' ']['X', 'X']],
            },
        ],
        cellumina::rule::EdgeBehaviour::Wrap,
    );

    // Rules can be converted to strings.
    // The rule can be recreated from the display output.
    let _rule2 = cellumina::rule::PatternRule::from(rule.to_string().as_str());

    // Therefore, you can save this output to a file and reload the rule later (or even type such a file yourself so the rule and patterns do not need to be created in code.)

    let path = "./tests/to_string/basic_rule.cel";

    std::fs::write(path, rule.to_string()).expect("Could not write to file!");

    let _rule3 = cellumina::rule::PatternRule::from(
        std::fs::read_to_string(path)
            .expect("Could not read file!")
            .as_str(),
    );
}
