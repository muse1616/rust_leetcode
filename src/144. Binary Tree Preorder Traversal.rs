impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![];
        let mut cur = root.clone();
        while !stack.is_empty() || cur.is_some(){
            while let Some(node) = cur{
                cur = node.borrow_mut().left.clone();
                ans.push(node.borrow().val);
                stack.push(node);
            }
            if let Some(node) = stack.pop(){
                cur = node.borrow_mut().right.clone();
            }
        }
        ans
    }
}