
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut cur = &mut result;

        // l1 l2 sum carry
        let mut t = (l1, l2, 0, 0);
        loop {
            t = match t {
                (None, None, _, 0) => break,
                (None, None, _, carry) => (None, None, carry, 0),
                (Some(l1), Some(l2), _, carry) if l1.val + l2.val + carry >= 10 => {
                    (l1.next, l2.next, (l1.val + l2.val + carry) % 10, 1)
                }
                (Some(l1), Some(l2), _, carry) => (l1.next, l2.next, l1.val + l2.val + carry, 0),
                (Some(list), None, _, carry) | (None, Some(list), _, carry)
                    if list.val + carry >= 10 =>
                {
                    (list.next, None, list.val + carry - 10, 1)
                }
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    (list.next, None, list.val + carry, 0)
                }
            };
            *cur = Some(Box::new(ListNode::new(t.2)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        result
    }
}

