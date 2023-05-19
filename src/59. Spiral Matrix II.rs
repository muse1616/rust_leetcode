impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];

        let (mut l, mut r, mut u, mut d) = (0 as i32, (n - 1) as i32, 0 as i32, (n - 1) as i32);
        let mut index = 1;
        loop {
            for i in l..=r {
                matrix[u as usize][i as usize] = index;
                index += 1;
            }
            u += 1;
            if u > d {
                break;
            }
            for i in u..=d {
                matrix[i as usize][r as usize] = index;
                index += 1;
            }
            r -= 1;
            if r < l {
                break;
            }
            for i in (l..=r).rev() {
                matrix[d as usize][i as usize] = index;
                index += 1;
            }
            d -= 1;
            if d < u {
                break;
            }
            for i in (u..=d).rev() {
                matrix[i as usize][l as usize] = index;
                index += 1;
            }
            l += 1;
            if l > r {
                break;
            }
        }

        matrix
    }
}