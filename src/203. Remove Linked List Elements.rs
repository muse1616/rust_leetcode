impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut dummy: Box<ListNode> = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut curr = &mut dummy;
        while curr.next.is_some() {
            if curr.next.as_ref().unwrap().val == val {
                curr.next = curr.next.as_mut().unwrap().next.take();
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}