impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut len = 0;
        let mut ptr = &head;
        while let Some(node) = ptr {
            len += 1;
            ptr = &node.next;
        }
        let mut ptr = &mut head;
        for _ in 0..len / 2 {
            if let Some(ref mut node) = ptr {
                ptr = &mut node.next;
            }
        }
        let mut next = ptr.take();
        let mut l1 = Self::sort_list(head);
        let mut l2 = Self::sort_list(next);
        Self::merge(l1, l2)
    }

    fn merge(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        match (l1, l2) {
            (None, Some(n)) | (Some(n), None) => Some(n),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    n1.next = Self::merge(n1.next.take(), Some(n2));
                    Some(n1)
                } else {
                    n2.next = Self::merge(Some(n1), n2.next.take());
                    Some(n2)
                }
            },
            (None, None) => None
        }
    }
}