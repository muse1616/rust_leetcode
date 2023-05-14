use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root == None { return 0; }
        let rb = root.as_ref().unwrap().borrow_mut();
        match (rb.left.as_ref(), rb.right.as_ref()) {
            (Some(m), None) => { 1 + Solution::depth(Some(m.clone())) },
            (None, Some(n)) => { 1 + Solution::depth(Some(n.clone())) },
            (Some(m), Some(n)) => { (1 + Solution::depth(Some(m.clone()))).max(1 + Solution::depth(Some(n.clone()))) },
            _ => { 1 },
        }
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // match root {
        //     Some(node) => {
        //         let left = Self::depth(node.borrow_mut().left.clone());
        //         let right = Self::depth(node.borrow_mut().right.clone());
        //         return (left - right).abs() <= 1
        //             && Self::is_balanced(node.borrow().left.clone())
        //             && Self::is_balanced(node.borrow().right.clone());
        //     }
        //     None => true,
        // }

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match root {
                Some(node) => {
                    let left = dfs(node.borrow().left.clone());
                    let right = dfs(node.borrow().right.clone());
                    if left == -1 || right == -1 || (left - right).abs() > 1 {
                        return -1;
                    }
                    1 + left.max(right)
                }
                None => 0,
            }
        }

        dfs(root) >= 0
    }
}