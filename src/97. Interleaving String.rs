impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let m = s1.len();
        let n = s2.len();
        let t = s3.len();
        if m + n != t {
            return false;
        }
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;

        for i in 0..=m {
            for j in 0..=n {
                let k = i + j - 1;
                if i > 0 {
                    dp[i][j] = dp[i][j] || (dp[i - 1][j] && s1[i - 1] == s3[k]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j] || (dp[i][j - 1] && s2[j - 1] == s3[k]);
                }
            }
        }

        dp[m][n]
    }
}
