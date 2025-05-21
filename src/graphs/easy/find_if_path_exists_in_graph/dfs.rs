use std::collections::{HashMap, HashSet};

pub fn valid_path(_n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for edge in edges {
        graph.entry(edge[0]).or_default().push(edge[1]);
        graph.entry(edge[1]).or_default().push(edge[0]);
    }

    let mut visited = HashSet::new();

    dfs(source, destination, &graph, &mut visited)
}

fn dfs(
    current: i32,
    destination: i32,
    graph: &HashMap<i32, Vec<i32>>,
    visited: &mut HashSet<i32>,
) -> bool {
    if current == destination {
        return true;
    }

    // Already seen
    if !visited.insert(current) {
        return false;
    }

    if let Some(neighbors) = graph.get(&current) {
        for &neighbor in neighbors {
            if dfs(neighbor, destination, graph, visited) {
                return true;
            }
        }
    }

    false
}
