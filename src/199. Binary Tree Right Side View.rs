
impl Solution {
    pub fn right_side_view(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        let mut queue = vec![];
        queue.push(root.take());
        while !queue.is_empty() {
            let n = queue.len();
            for i in 0..n {
                let node = queue[i].take();
                if i == n - 1 {
                    ans.push(node.as_ref().unwrap().borrow().val);
                }
                if node.as_ref().unwrap().borrow().left.is_some() {
                    queue.push(node.as_ref().unwrap().borrow_mut().left.take());
                }
                if node.as_ref().unwrap().borrow().right.is_some() {
                    queue.push(node.as_ref().unwrap().borrow_mut().right.take());
                }
            }
            queue = queue[n..].to_vec();
        }
        ans
    }
}