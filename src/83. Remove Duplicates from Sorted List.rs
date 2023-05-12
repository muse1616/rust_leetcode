impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none(){
            return head;
        }
        let mut curr = head.as_mut().unwrap();
        let mut duplicate = curr.val;
        while let Some(node) = curr.next.take(){
            if node.val == duplicate{
                // delete this node
                curr.next = node.next;
            }else{
                duplicate = node.val;
                curr.next = Some(node);
                curr = curr.next.as_mut().unwrap();
            }
        }

        head
    }
}