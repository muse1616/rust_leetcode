impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut flag_row = false;
        let mut flag_col = false;

        for i in 0..n {
            if matrix[0][i] == 0 {
                flag_row = true;
            }
        }
        for i in 0..m {
            if matrix[i][0] == 0 {
                flag_col = true;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0]  == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        if flag_col {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
        if flag_row {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}