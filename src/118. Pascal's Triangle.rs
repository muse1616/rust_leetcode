impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![]; num_rows as usize];
        for i in 0..num_rows as usize {
            ans[i].resize(i + 1, 0);
            if i == 0 {
                ans[0][0] = 1;
            } else {
                for j in 0..i + 1 {
                    if j == 0 || j == i {
                        ans[i][j] = 1;
                    } else {
                        ans[i][j] = ans[i - 1][j - 1] + ans[i - 1][j]
                    }
                }
            }
        }

        ans
    }
}