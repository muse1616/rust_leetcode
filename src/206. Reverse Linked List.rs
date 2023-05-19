impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // head insert
        // let mut pre = None;
        // let mut curr = head;
        // while let Some(mut node) = curr {
        //     curr = node.next.take();
        //     node.next = pre;
        //     pre = Some(node);
        // }

        // pre

        fn reverse(
            head: Option<Box<ListNode>>,
            prev: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut node) = head {
                let tail = node.next.take();
                node.next = prev;

                return reverse(tail, Some(node));
            }
            prev
        }

        reverse(head, None)
    }
}