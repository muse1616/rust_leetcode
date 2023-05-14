use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = Self::max_depth(node.borrow_mut().left.take());
                let right = Self::max_depth(node.borrow_mut().right.take());
                std::cmp::max(left, right) + 1
            }
            None => 0,
        }
    }
}