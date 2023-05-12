impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // normal
        // let mut index = 0;
        // for i in 0..nums.len(){
        //     if index == 0{
        //         nums[index] = nums[i];
        //         index+=1;
        //     }else{
        //         if nums[i] == nums[index-1]{
        //             continue;
        //         }else{
        //             nums[index] = nums[i];
        //             index+=1;
        //         }
        //     }
        // }

        // index as i32

        // functional style
        nums.dedup();
        nums.len() as i32
        
    }
}