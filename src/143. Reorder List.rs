
// impl Solution {
//     fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         // head insert
//         let mut pre = None;
//         let mut curr = head;
//         while let Some(mut node) = curr {
//             curr = node.next.take();
//             node.next = pre;
//             pre = Some(node);
//         }

//         pre
//     }
//     // half/reverse/merge
//     pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
//         let mut slow = head as *mut Option<Box<ListNode>>;
//         let mut fast = head as *mut Option<Box<ListNode>>;
//         unsafe {
//             while fast.as_ref().unwrap().is_some()
//                 && fast.as_ref().unwrap().as_ref().unwrap().next.is_some()
//                 && fast
//                     .as_ref()
//                     .unwrap()
//                     .as_ref()
//                     .unwrap()
//                     .next
//                     .as_ref()
//                     .unwrap()
//                     .next
//                     .is_some()
//             {
//                 fast = &mut fast.as_mut().unwrap().as_mut().unwrap().next;
//                 fast = &mut fast.as_mut().unwrap().as_mut().unwrap().next;
//                 slow = &mut slow.as_mut().unwrap().as_mut().unwrap().next;
//             }
//             let mut new_head = Self::reverse_list(slow.as_mut().unwrap().take());
//             // merge head and new_head
//             let mut res = ListNode::new(0);
//             let mut tail = &mut res;
//             let mut head = head.take();
//             while head.is_some() && new_head.is_some() {
//                 if head.is_some() {
//                     tail.next = head.take();
//                     tail = tail.next.as_mut().unwrap();
//                     head = tail.next.as_mut().unwrap().next.take();
//                 }
//                 if new_head.is_some() {
//                     tail.next = new_head.take();
//                     tail = tail.next.as_mut().unwrap();
//                     new_head = tail.next.as_mut().unwrap().next.take();
//                 }
//             }

//             head = res.next;
//         }
//     }
// }

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut buf = vec![];
        let mut node = head.take();
        while let Some(mut a) = node {
            node = a.next.take();
            buf.push(a);
        }

        let mut dump = Some(Box::new(ListNode::new(0)));
        let mut h = dump.as_mut();
        let n = buf.len();
        for i in 0..((n + 1) / 2) {
            let (l, r) = (i, n - i - 1);
            h.as_mut().unwrap().next = Some(buf[l].clone());
            h = h.unwrap().next.as_mut();
            if r > l {
                h.as_mut().unwrap().next = Some(buf[r].clone());
                h = h.unwrap().next.as_mut();
            }
        }
        *head = dump.unwrap().next;
    }
}
