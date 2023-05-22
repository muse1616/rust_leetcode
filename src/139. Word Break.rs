impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::HashSet;
        use std::iter::FromIterator;
        let s = s.as_bytes();
        let n = s.len();
        let mut set: HashSet<String> = HashSet::from_iter(word_dict.into_iter());
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && set.contains(&String::from_utf8(s[j..i].to_vec()).unwrap()) {
                    dp[i] = true;
                }
            }
        }

        dp[n]
    }
}