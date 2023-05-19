use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn backtrack(left: i32, right: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if left > right {
            return vec![None];
        }
        let mut all_trees = vec![];
        for i in left..=right {
            let left_trees = Self::backtrack(left, i - 1);
            let right_trees = Self::backtrack(i + 1, right);
            for left in left_trees.iter() {
                for right in right_trees.iter() {
                    let mut root = TreeNode {
                        val: i,
                        left: left.clone(),
                        right: right.clone(),
                    };
                    let mut root = Some(Rc::new(RefCell::new(root)));
                    all_trees.push(root);
                }
            }
        }
        all_trees
    }
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::backtrack(1, n)
    }
}