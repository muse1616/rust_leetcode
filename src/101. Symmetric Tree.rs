use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn check(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(p), Some(q)) => {
                return p.borrow().val == q.borrow().val
                    && Self::check(&p.borrow().left, &q.borrow().right)
                    && Self::check(&p.borrow().right, &q.borrow().left);
            }
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check(&root, &root)
    }
}