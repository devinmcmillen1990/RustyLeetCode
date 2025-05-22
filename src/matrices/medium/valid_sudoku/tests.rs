use crate::matrices::medium::valid_sudoku::{bitmasking_instead_of_hashsets, iteratively};

#[test]
fn test_is_valid_sudoku_cases_iteratively() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = iteratively::is_valid_sudoku(case.board.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

#[test]
fn test_is_valid_sudoku_cases_bitmasking_instead_of_hashsets() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = bitmasking_instead_of_hashsets::is_valid_sudoku(case.board.clone());
        assert_eq!(
            result, case.expected,
            "Test case {} failed: expected {}, got {}",
            i, case.expected, result
        );
    }
}

struct TestCase {
    board: Vec<Vec<char>>,
    expected: bool,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            board: vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            expected: true,
        },
        TestCase {
            board: vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'], // invalid: 8 repeated in box
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ],
            expected: false,
        },
        TestCase {
            board: vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            expected: true,
        },
        TestCase {
            board: vec![
                vec!['1', '1', '.', '.', '.', '.', '.', '.', '.'], // invalid: duplicate in row
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            expected: false,
        },
    ]
}
