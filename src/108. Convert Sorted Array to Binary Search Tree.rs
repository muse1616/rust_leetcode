use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&nums, 0, (nums.len() - 1) as i32)
    }
    fn build(nums: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let mid = left + (right - left) / 2;
        let mut node = TreeNode::new(nums[mid as usize]);
        node.left = Self::build(nums, left, mid - 1);
        node.right = Self::build(nums, mid + 1, right);
        return Some(Rc::new(RefCell::new(node)));
    }
}