impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        while let Some(mut node) = head {
            head = node.next.take();
            let mut ptr = &mut dummy;
            while let Some(t) = &ptr.next {
                if t.val >= node.val {
                    break;
                }
                ptr = ptr.next.as_mut().unwrap();
            }
            node.next = ptr.next.take();
            ptr.next = Some(node);
        }
        dummy.next
    }
}