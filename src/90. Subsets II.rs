impl Solution {
    fn dfs(
        nums: &Vec<i32>,
        cur: usize,
        tmp: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        choose_pre: bool,
    ) {
        if cur == nums.len() {
            ans.push(tmp.to_vec());
            return;
        }
        Self::dfs(nums, cur + 1, tmp, ans, false);
        if !choose_pre && cur > 0 && nums[cur] == nums[cur - 1] {
            return;
        }
        tmp.push(nums[cur]);
        Self::dfs(nums, cur + 1, tmp, ans, true);
        tmp.pop();
    }
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut tmp = vec![];
        let mut ans = vec![];
        Self::dfs(&nums, 0, &mut tmp, &mut ans, false);
        ans
    }
}
