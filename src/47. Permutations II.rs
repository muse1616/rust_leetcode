impl Solution {
    fn backtrack(mut nums: Vec<i32>, left: i32, right: i32, permutations: &mut Vec<Vec<i32>>) {
        if left == right {
            permutations.push(nums.clone());
            return;
        }
        for i in left..=right {
            let v1 = nums[left as usize];
            let v2 = nums[i as usize];
            if i != left && v1 == v2 {
                continue;
            }
            nums.swap(left as usize, i as usize);
            Self::backtrack(nums.clone(), left + 1, right, permutations);
            // nums.swap(i, index);
        }
    }
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut permutations = vec![];
        let right = nums.len() as i32 - 1;
        Self::backtrack(nums.clone(), 0, right, &mut permutations);
        permutations
    }
}