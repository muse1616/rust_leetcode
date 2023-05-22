
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    fn dfs(
        s: &[u8],
        index: i32,
        set: &HashSet<String>,
        dp: &Vec<bool>,
        path: &mut Vec<String>,
        ans: &mut Vec<String>,
    ) {
        if index == 0 {
            ans.push(path.join(" "));
            return;
        }
        for i in (0..=index - 1).rev() {
            let suffix = String::from_utf8(s[(i as usize)..(index as usize)].to_vec()).unwrap();
            if set.contains(&suffix) && dp[i as usize] {
                path.insert(0, suffix);
                Self::dfs(s, i, set, dp, path, ans);
                path.drain(0..1);
            }
        }
    }
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
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

        let mut res = vec![];
        if dp[n] {
            let mut path = Vec::new();
            Self::dfs(s, n as i32, &set, &dp, &mut path, &mut res);
            return res;
        }

        res
    }
}
