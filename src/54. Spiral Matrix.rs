impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut ans = Vec::with_capacity(n * m);
        let (mut l, mut r, mut u, mut d) = (0 as i32, (m - 1) as i32, 0 as i32, (n - 1) as i32);
        loop {
            for i in l..=r {
                ans.push(matrix[u as usize][i as usize]);
            }
            u += 1;
            if u > d {
                break;
            }
            for i in u..=d {
                ans.push(matrix[i as usize][r as usize]);
            }
            r -= 1;
            if r < l {
                break;
            }
            for i in (l..=r).rev() {
                ans.push(matrix[d as usize][i as usize]);
            }
            d -= 1;
            if d < u {
                break;
            }
            for i in (u..=d).rev() {
                ans.push(matrix[i as usize][l as usize]);
            }
            l += 1;
            if l > r {
                break;
            }
        }
        ans
    }
}