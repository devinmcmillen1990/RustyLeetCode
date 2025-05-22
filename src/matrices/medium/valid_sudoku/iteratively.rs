use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut boxes = vec![HashSet::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c == '.' {
                continue;
            }

            let b_idx = (i / 3) * 3 + j / 3;

            if rows[i].contains(&c) || cols[j].contains(&c) || boxes[b_idx].contains(&c) {
                return false;
            }

            rows[i].insert(c);
            cols[j].insert(c);
            boxes[b_idx].insert(c);
        }
    }
    true
}
