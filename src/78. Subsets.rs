impl Solution {
    fn dfs(nums: &Vec<i32>, cur: usize, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if cur == nums.len() {
            ans.push(tmp.to_vec());
            return;
        }
        tmp.push(nums[cur]);
        Self::dfs(nums, cur + 1, tmp, ans);
        tmp.pop();
        Self::dfs(nums, cur + 1, tmp, ans);
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut tmp = vec![];
        let mut ans = vec![];
        Self::dfs(&nums, 0, &mut tmp, &mut ans);
        ans
    }
}
