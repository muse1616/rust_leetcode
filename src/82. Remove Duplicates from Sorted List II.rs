impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut dummy: Box<ListNode> = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut curr = &mut dummy;
        while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
            if curr.next.as_ref().unwrap().val
                == curr.next.as_ref().unwrap().next.as_ref().unwrap().val
            {
                let v = curr.next.as_ref().unwrap().val;
                while curr.next.is_some() && curr.next.as_ref().unwrap().val == v {
                    curr.next = curr.next.as_mut().unwrap().next.take();
                }
            } else {
                curr = curr.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
