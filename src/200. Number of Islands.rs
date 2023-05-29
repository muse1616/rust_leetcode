impl Solution {
    fn dfs(i: i32, j: i32, grid: &mut Vec<Vec<char>>) {
        if i < 0
            || i == grid.len() as i32
            || j < 0
            || j == grid[0].len() as i32
            || grid[i as usize][j as usize] != '1'
        {
            return;
        }
        grid[i as usize][j as usize] = '0';
        Self::dfs(i + 1, j, grid);
        Self::dfs(i - 1, j, grid);
        Self::dfs(i, j + 1, grid);
        Self::dfs(i, j - 1, grid);
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    ans += 1;
                    Self::dfs(i as i32, j as i32, &mut grid);
                }
            }
        }
        ans
    }
}
