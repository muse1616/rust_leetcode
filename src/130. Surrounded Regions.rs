impl Solution {
    fn dfs(x: i32, y: i32, board: &mut Vec<Vec<char>>) {
        if x < 0
            || x == board.len() as i32
            || y < 0
            || y == board[0].len() as i32
            || board[x as usize][y as usize] != 'O'
        {
            return;
        }
        board[x as usize][y as usize] = 'o';
        Self::dfs(x - 1, y, board);
        Self::dfs(x + 1, y, board);
        Self::dfs(x, y - 1, board);
        Self::dfs(x, y + 1, board);
    }
    pub fn solve(board: &mut Vec<Vec<char>>) {
        for i in 0..board.len() {
            Self::dfs(i as i32, 0, board);
            Self::dfs(i as i32, (board[0].len() - 1) as i32, board);
        }
        for i in 0..board[0].len() {
            Self::dfs(0 as i32, i as i32, board);
            Self::dfs((board.len() - 1) as i32, i as i32, board);
        }
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'o' {
                    board[i][j] = 'O'
                }
            }
        }
    }
}
