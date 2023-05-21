impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let mut root = TreeNode::new(preorder[0]);
        let mut index = 0;
        for (k, v) in inorder.iter().enumerate() {
            if *v == preorder[0] {
                index = k;
                break;
            }
        }

        root.left = Self::build_tree(preorder[1..=index].to_vec(), inorder[..index].to_vec());
        root.right = Self::build_tree(
            preorder[index + 1..].to_vec(),
            inorder[index + 1..].to_vec(),
        );

        Some(Rc::new(RefCell::new(root)))
    }
}
