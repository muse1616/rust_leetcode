impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_by(|a, b| {
            format!("{}{}", b, a)
                .partial_cmp(&format!("{}{}", a, b))
                .unwrap()
        });
        if nums.len() > 1 && nums[0] == "0"{
            return String::from("0");
        }
        nums.join("")
    }
}