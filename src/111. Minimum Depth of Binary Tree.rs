// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                let min = i32::MAX;
                match (left, right) {
                    (None, None) => 1,
                    (None, Some(node)) | (Some(node), None) => min.min(Self::min_depth(Some(node))) + 1,
                    (Some(left), Some(right)) => {
                        min.min(Self::min_depth(Some(left)).min(Self::min_depth(Some(right)))) + 1
                    }
                }
            }
            None => 0,
        }
    }
}