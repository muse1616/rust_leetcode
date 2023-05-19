impl Solution {
    fn dfs(
        i: i32,
        j: i32,
        index: usize,
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if index as usize == word.len() {
            return true;
        }
        if i < 0
            || i == board.len() as i32
            || j < 0
            || j == board[0].len() as i32
            || word[index] != board[i as usize][j as usize]
            || visited[i as usize][j as usize]
        {
            return false;
        }
        visited[i as usize][j as usize] = true;
        let res = Self::dfs(i + 1, j, index + 1, board, word, visited)
            || Self::dfs(i, j + 1, index + 1, board, word, visited)
            || Self::dfs(i - 1, j, index + 1, board, word, visited)
            || Self::dfs(i, j - 1, index + 1, board, word, visited);
        visited[i as usize][j as usize] = false;
        res
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == word[0] {
                    if Self::dfs(i as i32, j as i32, 0, &board, &word, &mut visited) {
                        return true;
                    }
                }
            }
        }
        false
    }
}