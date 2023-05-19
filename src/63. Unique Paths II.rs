impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut grid = vec![vec![0; n]; m];
        grid[0][0] = 1 - obstacle_grid[0][0];

        for i in 0..m as usize {
            for j in 0..n as usize {
                if obstacle_grid[i][j] == 1 {
                    grid[i][j] = 0;
                    continue;
                }
                if i == 0 || j == 0 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    if i == 0 {
                        grid[i][j] = grid[i][j - 1];
                    } else {
                        grid[i][j] = grid[i - 1][j];
                    }
                } else {
                    grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
                }
            }
        }

        grid[m - 1][n - 1]
    }
}