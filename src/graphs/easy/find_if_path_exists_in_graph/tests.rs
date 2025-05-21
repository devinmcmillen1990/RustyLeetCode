use crate::graphs::easy::find_if_path_exists_in_graph::{bfs, dfs, union_find_disjoint_set_union};

#[test]
fn test_valid_path_dfs() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = dfs::valid_path(case.n, case.edges.clone(), case.source, case.destination);
        assert_eq!(
            result, case.expected,
            "DFS failed at case {}: input=({}, {:?}, {}, {}), expected {}, got {}",
            i, case.n, case.edges, case.source, case.destination, case.expected, result
        );
    }
}

#[test]
fn test_valid_path_bfs() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = bfs::valid_path(case.n, case.edges.clone(), case.source, case.destination);
        assert_eq!(
            result, case.expected,
            "BFS failed at case {}: input=({}, {:?}, {}, {}), expected {}, got {}",
            i, case.n, case.edges, case.source, case.destination, case.expected, result
        );
    }
}

#[test]
fn test_valid_path_union_find() {
    for (i, case) in get_test_cases().into_iter().enumerate() {
        let result = union_find_disjoint_set_union::valid_path(
            case.n,
            case.edges.clone(),
            case.source,
            case.destination,
        );
        assert_eq!(
            result, case.expected,
            "Union-Find failed at case {}: input=({}, {:?}, {}, {}), expected {}, got {}",
            i, case.n, case.edges, case.source, case.destination, case.expected, result
        );
    }
}

#[cfg(test)]
struct TestCase {
    n: i32,
    edges: Vec<Vec<i32>>,
    source: i32,
    destination: i32,
    expected: bool,
}

fn get_test_cases() -> Vec<TestCase> {
    vec![
        TestCase {
            n: 3,
            edges: vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            source: 0,
            destination: 2,
            expected: true,
        },
        TestCase {
            n: 6,
            edges: vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            source: 0,
            destination: 5,
            expected: false,
        },
        TestCase {
            n: 2,
            edges: vec![],
            source: 0,
            destination: 1,
            expected: false,
        },
        TestCase {
            n: 1,
            edges: vec![],
            source: 0,
            destination: 0,
            expected: true,
        },
        TestCase {
            n: 4,
            edges: vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            source: 0,
            destination: 3,
            expected: true,
        },
    ]
}
