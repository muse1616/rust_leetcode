impl Solution {
    fn get_middle(head:Option<Box<ListNode>>,tail:Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        let mut slow = &head;
        let mut fast = &head;
        let tail_ref = &tail;
        while fast != tail_ref && fast.as_ref().unwrap().next != tail{
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
        }
        slow.clone()    
    }
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::get_middle(head, None)
    }
}