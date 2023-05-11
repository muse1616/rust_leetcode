/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 *
 * https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (39.11%)
 * Likes:    9127
 * Dislikes: 0
 * Total Accepted:    2.3M
 * Total Submissions: 5.9M
 * Testcase Example:  '"abcabcbb"'
 *
 * Given a string s, find the length of the longest substring without repeating
 * characters.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not
 * a substring.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= s.length <= 5 * 10^4
 * s consists of English letters, digits, symbols and spaces.
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        let mut last : [i32;128]= [-1;128];
        let mut len = 0;
        let mut left = -1;
        for (i,v) in s.chars().enumerate(){
            left = max(left,last[v as usize] as i32);
            last[v as usize] = i as i32;
            len = std::cmp::max(len, i as i32 - left)
        }
        len

    }
}
// @lc code=end

