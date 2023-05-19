impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut small = Box::new(ListNode::new(-1));
        let mut large = Box::new(ListNode::new(-1));
        let mut small_ptr = &mut small;
        let mut large_ptr = &mut large;
        while let Some(mut curr) = head {
            head = curr.next.take();
            if curr.val < x {
                small_ptr.next = Some(curr);
                small_ptr = small_ptr.next.as_mut().unwrap();
            } else {
                large_ptr.next = Some(curr);
                large_ptr = large_ptr.next.as_mut().unwrap();
            }
        }
        small_ptr.next = large.next.take();
        small.next
    }
}