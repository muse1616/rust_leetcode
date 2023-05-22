impl Solution {
    pub fn postorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut res = vec![];
        let mut prev = None;
        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow_mut().left.take();
                stack.push(node);
            }
            root = stack.pop();
            if let Some(n) = root {
                if n.borrow().right.is_none() || n.borrow().right == prev {
                    res.push(n.borrow().val);
                    prev = Some(n);
                    root = None;
                } else {
                    stack.push(n.clone());
                    root = n.borrow().right.clone();
                }
            }
        }

        res
    }
}