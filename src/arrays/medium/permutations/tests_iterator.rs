use crate::arrays::medium::permutations::lazy_yield_iterator::Permutations;

#[cfg(test)]
struct TestCase {
    input: Vec<i32>,
    expected_first_n: Vec<Vec<i32>>,
    expected_count: usize,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            input: vec![1, 2, 3],
            expected_first_n: vec![vec![1, 2, 3], vec![2, 1, 3], vec![3, 1, 2]],
            expected_count: 6,
        },
        TestCase {
            input: vec![0, 1],
            expected_first_n: vec![vec![0, 1], vec![1, 0]],
            expected_count: 2,
        },
        TestCase {
            input: vec![42],
            expected_first_n: vec![vec![42]],
            expected_count: 1,
        },
    ]
}

#[test]
fn test_permutations_iterator() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let iter = Permutations::new(case.input.clone());
        let output: Vec<Vec<i32>> = iter.collect();

        assert_eq!(
            output.len(),
            case.expected_count,
            "Case {} failed: expected {} permutations, got {}",
            i,
            case.expected_count,
            output.len()
        );

        for (j, expected_perm) in case.expected_first_n.iter().enumerate() {
            assert_eq!(
                &output[j], expected_perm,
                "Case {} failed at permutation {}: expected {:?}, got {:?}",
                i, j, expected_perm, output[j]
            );
        }
    }
}
