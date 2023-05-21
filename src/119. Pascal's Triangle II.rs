impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut ans: Vec<Vec<i32>> = vec![vec![]; (row_index+1) as usize];
        for i in 0..(row_index+1) as usize {
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

        ans[row_index as usize].clone()
    }
}