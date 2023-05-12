
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m = HashMap::new();
        for (index,value) in nums.iter().enumerate() {
            if m.contains_key(&(target-value)){
                return vec![index as i32,m[&(target-value)]];
            }
            m.insert(value, index as i32);
        }
        return vec![];
    }
}

