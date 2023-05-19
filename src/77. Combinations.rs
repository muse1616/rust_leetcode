impl Solution {
    fn dfs(n: i32, k: i32, cur: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if tmp.len() as i32 + n - cur + 1 < k {
            return;
        }
        if tmp.len() as i32 == k {
            ans.push(tmp.to_vec());
            return;
        }
        if cur == n + 1 {
            return;
        }
        tmp.push(cur);
        Self::dfs(n, k, cur + 1, tmp, ans);
        tmp.pop();
        Self::dfs(n, k, cur + 1, tmp, ans);
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut tmp = vec![];

        Self::dfs(n, k, 1, &mut tmp, &mut ans);
        ans
    }
}
