use std::collections::{HashMap, HashSet, VecDeque};

pub fn valid_path(_n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    for edge in edges {
        graph.entry(edge[0]).or_default().push(edge[1]);
        graph.entry(edge[1]).or_default().push(edge[0]);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(source);

    while let Some(node) = queue.pop_front() {
        if node == destination {
            return true;
        }

        if visited.insert(node) {
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    false
}
