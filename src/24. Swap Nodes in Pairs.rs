impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut node| match node.next {
            Some(mut next) => {
                node.next = Self::swap_pairs(next.next);
                next.next = Some(node);
                Some(next)
            }
            None => Some(node),
        })
    }
}