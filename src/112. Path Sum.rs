impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, rest: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let val = root.as_ref().unwrap().borrow().val;

        if root.as_ref().unwrap().borrow().left.is_none()
            && root.as_ref().unwrap().borrow().right.is_none()
        {
            if rest == val {
                return true;
            }
            return false;
        }
        let mut root = root.as_ref().unwrap().borrow_mut();
        Self::dfs(root.left.take(), rest - val) || Self::dfs(root.right.take(), rest - val)
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::dfs(root, target_sum)
    }
}
