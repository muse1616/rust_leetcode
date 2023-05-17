impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // normal
        let mut row: [[u8; 9]; 9] = [[0; 9]; 9];
        let mut col: [[u8; 9]; 9] = [[0; 9]; 9];
        let mut sub: [[[u8; 9]; 3]; 3] = [[[0; 9]; 3]; 3];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] >= '0' && board[i][j] <= '9' {
                    let c = board[i][j] as u8 - b'1';
                    row[i][c as usize] += 1;
                    col[j][c as usize] += 1;
                    sub[i / 3][j / 3][c as usize] += 1;
                    if row[i][c as usize] > 1
                        || col[j][c as usize] > 1
                        || sub[i / 3][j / 3][c as usize] > 1
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}