impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // normal
        // let mut index = 0;
        // for i in 0..nums.len() {
        //     if nums[i]!=val{
        //         nums[index]=nums[i];
        //         index+=1;
        //     }
        // }
        // index as i32

        // functional style
        nums.retain(|&x|x!=val);
        nums.len() as i32
    }
}