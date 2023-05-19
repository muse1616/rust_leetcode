impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                } else if i == 0 {
                    grid[i][j] += grid[i][j - 1];
                } else if j == 0 {
                    grid[i][j] += grid[i - 1][j];
                } else {
                    grid[i][j] += grid[i - 1][j].min(grid[i][j - 1])
                }
            }
        }

        grid[m - 1][n - 1]
    }
}