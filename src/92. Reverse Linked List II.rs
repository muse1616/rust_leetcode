impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut head = head;
        if left == 1 {
            return Self::reverse_list_n(head, right);
        }
        head.as_mut().unwrap().next =
            Self::reverse_between(head.as_mut().unwrap().next.take(), left - 1, right - 1);
        head
    }
    pub fn reverse_list_n(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut k = k;
        let mut tail: Option<Box<ListNode>> = None;
        let mut new_head = None;
        {
            while let Some(mut node) = head {
                if k == 0 {
                    tail = Some(node);
                    break;
                }
                head = node.next.take();
                node.next = new_head;
                new_head = Some(node);
                k -= 1;
            }
        }
        let mut cur = &mut new_head;

        while cur.as_ref().unwrap().next.is_some() {
            cur = &mut cur.as_mut().unwrap().next;
        }

        cur.as_mut().unwrap().next = tail;
        new_head
    }
}
