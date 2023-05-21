impl Solution {
    // timeout
    // fn get_middle(
    //     head: &Option<Box<ListNode>>,
    //     tail: &Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     let mut slow = head;
    //     let mut fast = head;
    //     while fast.clone() != tail.clone() && fast.as_ref().unwrap().next != tail.clone() {
    //         slow = &slow.as_ref().unwrap().next;
    //         fast = &fast.as_ref().unwrap().next;
    //         fast = &fast.as_ref().unwrap().next;
    //     }
    //     slow.clone()
    // }

    // fn build(
    //     head: &mut Option<Box<ListNode>>,
    //     tail: &mut Option<Box<ListNode>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     if head == tail {
    //         return None;
    //     }
    //     let mut mid = Self::get_middle(head, tail);

    //     let mut root = TreeNode::new(mid.as_ref().unwrap().val);
    //     root.left = Self::build(head, &mut mid);
    //     root.right = Self::build(&mut mid.as_mut().unwrap().next, tail);
    //     Some(Rc::new(RefCell::new(root)))
    // }
    // pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     Self::build(&mut head, &mut None)
    // }

    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        // 收集链表节点值
        while let Some(node) = head {
            stack.push(node.val);
            head = node.next;
        }
        Solution::s(&stack[..])
    }
    pub fn s(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut n = nums.len();
        if n == 0 {
            return None;
        }
        let mid = n / 2;
        let mut node = TreeNode::new(nums[mid]);
        // 递归创建左子树
        node.left = Solution::s(&nums[..mid]);
        // 递归创建右子树
        node.right = Solution::s(&nums[mid + 1..]);
        Some(Rc::new(RefCell::new(node)))
    }
}