impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> i32 {
        match node {
            Some(node) => {
                let node = node.as_ref().borrow();
                sum = sum * 10 + node.val;
                if node.left.is_none() && node.right.is_none() {
                    return sum;
                }
                return Self::dfs(&node.left, sum) + Self::dfs(&node.right, sum);
            }
            None => 0,
        }
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 0)
    }
}
