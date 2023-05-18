impl Solution {
    fn backtrack(nums: &mut Vec<i32>, index: usize, permutations: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            permutations.push(nums.to_vec());
            return;
        }
        for i in index..nums.len() {
            nums.swap(i, index);
            Self::backtrack(nums, index + 1, permutations);
            nums.swap(i, index);
        }
    }
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = vec![];
        Self::backtrack(&mut nums, 0, &mut permutations);
        permutations
    }
}