impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right): (usize, usize) = (0, height.len() - 1);
        let mut ans = 0;
        while left < right {
            ans = ans.max((right as i32 - left as i32) * (height[left].min(height[right])));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}