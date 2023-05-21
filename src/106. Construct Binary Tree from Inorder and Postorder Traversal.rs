impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None;
        }
        let mut root = TreeNode::new(*postorder.last().unwrap());
        let mut index = 0;
        for (k, v) in inorder.iter().enumerate() {
            if *v == root.val {
                index = k;
                break;
            }
        }

        root.left = Self::build_tree(
            inorder[..index].to_vec().to_vec(),
            postorder[0..index].to_vec(),
        );
        root.right = Self::build_tree(
            inorder[index + 1..].to_vec().to_vec(),
            postorder[index..postorder.len() - 1].to_vec(),
        );

        Some(Rc::new(RefCell::new(root)))
    }
}
