use std::cell::RefCell;
use std::rc::Rc;
struct BSTIterator {
    nodes: Vec<i32>,
    index: RefCell<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn inorder(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(node) => {
                self.inorder(node.borrow().left.clone());
                self.nodes.push(node.borrow().val);
                self.inorder(node.borrow().right.clone());
            }
            None => {}
        }
    }
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = Self {
            nodes: vec![],
            index: RefCell::new(0),
        };
        iter.inorder(root);
        iter
    }

    fn next(&self) -> i32 {
        let mut index = self.index.borrow_mut();
        let n = &self.nodes[*index];
        *index += 1;
        *n
    }

    fn has_next(&self) -> bool {
        *self.index.borrow() < self.nodes.len()
    }
}
