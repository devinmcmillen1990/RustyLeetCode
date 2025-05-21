use crate::math::easy::find_the_winning_player_in_coin_game::mathy::find_winner;

#[test]
fn test_find_winner() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = find_winner(case.x, case.y);
        assert_eq!(
            result, case.expected,
            "Test case {} failed: x={}, y={}, expected {}, got {}",
            i, case.x, case.y, case.expected, result
        );
    }
}

#[cfg(test)]
struct TestCase {
    x: i32,
    y: i32,
    expected: &'static str,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            x: 1,
            y: 4,
            expected: "Alice",
        },
        TestCase {
            x: 2,
            y: 4,
            expected: "Alice",
        },
        TestCase {
            x: 3,
            y: 4,
            expected: "Alice",
        },
        TestCase {
            x: 4,
            y: 8,
            expected: "Bob",
        },
        TestCase {
            x: 5,
            y: 16,
            expected: "Bob",
        },
        TestCase {
            x: 0,
            y: 4,
            expected: "Bob",
        },
        TestCase {
            x: 1,
            y: 3,
            expected: "Bob",
        },
        TestCase {
            x: 2,
            y: 8,
            expected: "Bob",
        },
        TestCase {
            x: 3,
            y: 8,
            expected: "Bob",
        },
        TestCase {
            x: 7,
            y: 16,
            expected: "Bob",
        },
    ]
}
