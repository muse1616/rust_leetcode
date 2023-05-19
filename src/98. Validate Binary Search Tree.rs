use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn check(root: &Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
        if root.is_none() {
            return true;
        }
        let val = root.as_ref().unwrap().borrow().val as i64;
        if val <= lower || val >= upper {
            return false;
        }

        Self::check(&root.as_ref().unwrap().borrow().left, lower, val)
            && Self::check(&root.as_ref().unwrap().borrow().right, val, upper)
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check(&root, i64::MIN, i64::MAX)
    }
}