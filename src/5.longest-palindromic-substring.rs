impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![vec![false; n ]; n];
        let (mut start, mut end) = (0, 0);
        for i in (0..s.len()).rev() {
            for j in i..s.len() {
                if i == j || (j - i == 1 && s[i] == s[j]) {
                    dp[i][j] = true;
                }else  {
                    dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];
                }
                if dp[i][j] && j - i > end-start {
                    start = i;
                    end = j;
                }
            }
        }

        s[start..=end].iter().collect()
    }
}

