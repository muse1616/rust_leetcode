impl Solution {
    fn dfs(
        s: &[u8],
        i: usize,
        current: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
        f: &Vec<Vec<bool>>,
    ) {
        if i == s.len() {
            ans.push(current.to_vec());
            return;
        }
        for j in i..s.len() {
            if f[i][j] {
                current.push(String::from_utf8(s[i..=j].to_vec()).unwrap());
                Self::dfs(s, j+1, current, ans, f);
                current.pop();
            }
        }
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let n = s.len();
        let mut f = vec![vec![true; n]; n];
        let mut ans = vec![];
        for i in (0..=n - 1).rev() {
            for j in (i + 1..n) {
                f[i][j] = f[i + 1][j - 1] && s[i] == s[j];
            }
        }
        let mut current = vec![];
        Self::dfs(s, 0, &mut current, &mut ans, &f);
        ans
    }
}
