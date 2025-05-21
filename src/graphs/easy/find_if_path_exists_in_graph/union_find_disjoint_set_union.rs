pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut parent: Vec<usize> = (0..n as usize).collect();

    for edge in edges {
        union(edge[0] as usize, edge[1] as usize, &mut parent);
    }

    find(source as usize, &mut parent) == find(destination as usize, &mut parent)
}

fn union(x: usize, y: usize, parent: &mut Vec<usize>) {
    let px = find(x, parent);
    let py = find(y, parent);
    if px != py {
        parent[py] = px;
    }
}

fn find(x: usize, parent: &mut Vec<usize>) -> usize {
    if parent[x] != x {
        parent[x] = find(parent[x], parent);
    }
    parent[x]
}
