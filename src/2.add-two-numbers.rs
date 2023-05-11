/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.cn/problems/add-two-numbers/description/
 *
 * algorithms
 * Medium (42.39%)
 * Likes:    9573
 * Dislikes: 0
 * Total Accepted:    1.7M
 * Total Submissions: 4.1M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 * 
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 * 
 * 
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut cur = &mut result;

        // l1 l2 sum carry
        let mut t = (l1, l2, 0, 0);
        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(l1), Some(l2), _, carry) if l1.val + l2.val + carry >= 10 => {
                    (l1.next, l2.next, (l1.val + l2.val + carry) % 10, 1)
                }
                (Some(l1), Some(l2), _, carry) => (l1.next, l2.next, l1.val + l2.val + carry, 0),
                (Some(list), None, _, carry) | (None, Some(list), _, carry)
                    if list.val + carry >= 10 =>
                {
                    (list.next, None, list.val + carry - 10, 1)
                }
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    (list.next, None, list.val + carry, 0)
                }
            };
            *cur = Some(Box::new(ListNode::new(t.2)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        result
    }
}
// @lc code=end

