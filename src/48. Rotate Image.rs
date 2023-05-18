impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // 上下 对角
        let n = matrix.len();
        let mut i = 0;
        let mut j = n - 1;
        while i < j {
            matrix.swap(i, j);
            i += 1;
            j -= 1;
        }
        for i in 0..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}