use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut curr = root.clone();
        while let Some(node) = curr {
            let mut node = node.borrow_mut();
            if let Some(next_node) = node.left.take() {
                let mut pre = next_node.clone();
                let mut pre_right = pre.borrow_mut().right.clone();
                while let Some(node) = pre_right {
                    pre_right = node.borrow_mut().right.clone();
                    pre = node;
                }
                pre.borrow_mut().right = node.right.take();
                node.right = Some(next_node);
            }
            curr = node.right.clone();
        }
    }
}