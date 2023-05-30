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
    fn count_levels(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level = 0;
        while let Some(node) = root {
            level += 1;
            root = node.borrow().left.clone();
        }
        level
    }
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let left = Self::count_levels(root.as_ref().unwrap().borrow().left.clone());
        let right = Self::count_levels(root.as_ref().unwrap().borrow().right.clone());
        if left == right {
            return Self::count_nodes(root.as_ref().unwrap().borrow().right.clone()) + (1 << left);
        } else {
            return Self::count_nodes(root.as_ref().unwrap().borrow().left.clone()) + (1 << right);
        }
    }
}