impl Solution {
    fn dfs(curr: i32, sum: i32, arr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, k: i32) {
        if arr.len() as i32 + (9 - curr + 1) < k || arr.len() > k as usize {
            return;
        }
        if arr.len() == k as usize && sum == 0 {
            ans.push(arr.to_vec());
            return;
        }
        arr.push(curr);
        Self::dfs(curr + 1, sum - curr, arr, ans, k);
        arr.pop();
        Self::dfs(curr + 1, sum, arr, ans, k);
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut arr = vec![];
        let mut ans = vec![];
        Self::dfs(1, n, &mut arr, &mut ans, k);
        ans
    }
}