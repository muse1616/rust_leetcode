/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 *
 * https://leetcode.cn/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (37.54%)
 * Likes:    6478
 * Dislikes: 0
 * Total Accepted:    1.4M
 * Total Submissions: 3.8M
 * Testcase Example:  '"babad"'
 *
 * Given a string s, return the longest palindromic substring in s.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "cbbd"
 * Output: "bb"
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= s.length <= 1000
 * s consist of only digits and English letters.
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![vec![false; n ]; n];
        let (mut start, mut end) = (0, 0);
        for i in (0..s.len()).rev() {
            for j in i..s.len() {
                if i == j || (j - i == 1 && s[i] == s[j]) {
                    dp[i][j] = true;
                }else  {
                    dp[i][j] = dp[i + 1][j - 1] && s[i] == s[j];
                }
                if dp[i][j] && j - i > end-start {
                    start = i;
                    end = j;
                }
            }
        }

        s[start..=end].iter().collect()
    }
}
// @lc code=end

