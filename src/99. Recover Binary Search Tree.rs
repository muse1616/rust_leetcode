use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        let mut cur = root.clone();
        let mut left = None;
        let mut right = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while !stack.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow_mut().left.clone();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                if let Some(p) = prev {
                    if p.borrow_mut().val > node.borrow_mut().val {
                        right = Some(node.clone());

                        if left.is_none() {
                            left = Some(p);
                        } else {
                            break;
                        }
                    }
                }

                prev = Some(node.clone());
                cur = node.borrow_mut().right.clone();
            }
        }

        swap(
            &mut left.unwrap().borrow_mut().val,
            &mut right.unwrap().borrow_mut().val,
        )
    }
}
