use crate::dynamic_programming::easy::best_time_to_buy_and_sell_stock::{
    brute_force, greedy_with_dp,
};

#[test]
fn test_max_profit_brute_force() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = brute_force::max_profit(case.prices.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: prices = {:?}, expected {}, got {}",
            i, case.prices, case.expected, result
        );
    }
}

#[test]
fn test_max_profit_greedy_with_dp() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = greedy_with_dp::max_profit(case.prices.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: prices = {:?}, expected {}, got {}",
            i, case.prices, case.expected, result
        );
    }
}

struct TestCase {
    prices: Vec<i32>,
    expected: i32,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            prices: vec![7, 1, 5, 3, 6, 4],
            expected: 5,
        },
        TestCase {
            prices: vec![7, 6, 4, 3, 1],
            expected: 0,
        },
        TestCase {
            prices: vec![2, 4, 1],
            expected: 2,
        },
        TestCase {
            prices: vec![1],
            expected: 0,
        },
        TestCase {
            prices: vec![],
            expected: 0,
        },
    ]
}
