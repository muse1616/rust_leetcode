impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        unsafe {
            let mut fast = &mut dummy as *mut Box<ListNode>;
            let mut slow = &mut dummy as *mut Box<ListNode>;
            for i in 0..n {
                fast = (*fast).next.as_mut().unwrap();
            }
            while (*fast).next.is_some() {
                fast = (*fast).next.as_mut().unwrap();
                slow = (*slow).next.as_mut().unwrap();
            }
            (*slow).next = (*slow).next.take().unwrap().next;
        }

        dummy.next
    }
}