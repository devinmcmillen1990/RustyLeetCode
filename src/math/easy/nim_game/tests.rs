use crate::math::easy::nim_game::can_win::can_win_nim;

#[test]
fn test_can_win_nim_cases() {
    let test_cases = vec![
        (1, true),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (6, true),
        (7, true),
        (8, false),
        (12, false),
        (13, true),
        (20, false),
        (21, true),
        (100, false),
        (101, true),
    ];

    for (input, expected) in test_cases {
        assert_eq!(can_win_nim(input), expected, "Failed for input: {}", input);
    }
}
