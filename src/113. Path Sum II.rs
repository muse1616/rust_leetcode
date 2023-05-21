impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        rest: i32,
        mut path: Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if root.is_none() {
            return;
        }
        let mut root = root.as_ref().unwrap().borrow_mut();
        let val = root.val;
        if root.left.is_none() && root.right.is_none() {
            if rest == val {
                path.push(val);
                ans.push(path.to_vec());
            }
            return;
        }
        path.push(val);

        Self::dfs(root.left.take(), rest - val, path.clone(), ans);
        Self::dfs(root.right.take(), rest - val, path.clone(), ans);
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let path = vec![];
        Self::dfs(root, target_sum, path, &mut ans);
        ans
    }
}
