
impl Solution {
    pub fn level_order_bottom(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = vec![];
        if root.is_none() {
            return levels;
        }
        let mut queue = vec![];
        queue.push(root.take());
        while !queue.is_empty() {
            let n = queue.len();
            let mut level = vec![];
            for i in 0..n {
                let node = queue[i].take();
                level.push(node.as_ref().unwrap().borrow().val);
                if node.as_ref().unwrap().borrow().left.is_some() {
                    queue.push(node.as_ref().unwrap().borrow_mut().left.take());
                }
                if node.as_ref().unwrap().borrow().right.is_some() {
                    queue.push(node.as_ref().unwrap().borrow_mut().right.take());
                }
            }
            levels.push(level);
            // levels.insert(0, level);
            queue = queue[n..].to_vec();
        }
        levels.reverse();
        levels
    }
}