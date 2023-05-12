impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let (mut list1, mut list2) = (list1, list2);
        // let mut head : Option<Box<ListNode>>= None;
        // let mut cur: &mut Option<Box<ListNode>> = &mut head;
        // loop{
        //     match (list1,list2){
        //         (Some(mut a),Some(mut b))=>{
        //             if a.val < b.val{
        //                 list1 = a.next.take();
        //                 list2 = Some(b);
        //                 cur = &mut cur.insert(a).next;
        //             }else{
        //                 list2 = b.next.take();
        //                 list1 = Some(a);
        //                 cur = &mut cur.insert(b).next;
        //             }
        //         }
        //         (x,y)=>{*cur = x.or(y);break;}
        //     }
        // }
        // head

        let (mut list1, mut list2) = (list1, list2);
        let mut dummy = ListNode::new(-1);
        let mut cur = &mut dummy;
        while let (Some(l1),Some(l2)) = (list1.as_ref(),list2.as_ref()) {
            let l = if l1.val < l2.val {&mut list1} else {&mut list2};

            cur.next = l.take();
            cur = cur.next.as_mut().unwrap();
            *l = cur.next.take();
        }
        cur.next = list1.or(list2);
        dummy.next

    }
}