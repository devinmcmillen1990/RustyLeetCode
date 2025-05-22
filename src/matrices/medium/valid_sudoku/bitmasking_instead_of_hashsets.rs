pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = [0u16; 9];
    let mut cols = [0u16; 9];
    let mut boxes = [0u16; 9];

    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c == '.' {
                continue;
            }

            let bit = 1 << (c as u8 - b'1');
            let box_idx = (i / 3) * 3 + j / 3;

            if rows[i] & bit != 0 || cols[j] & bit != 0 || boxes[box_idx] & bit != 0 {
                return false;
            }

            rows[i] |= bit;
            cols[j] |= bit;
            boxes[box_idx] |= bit;
        }
    }

    true
}
