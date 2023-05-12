use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            Some(node) => {
                Self::inorder(node.borrow().left.clone(), res);
                res.push(node.borrow().val);
                Self::inorder(node.borrow().right.clone(), res);
            }
            None => {}
        }
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut ans = vec![];
        // Self::inorder(root, &mut ans);
        // ans

        // let mut ans = vec![];
        // let mut stack = vec![];
        // while root.is_some() || !stack.is_empty(){
        //     while let Some(node) = root{
        //         root = node.borrow_mut().left.take();
        //         stack.push(node);
        //     }
        //     if let Some(node) = stack.pop(){
        //         ans.push(node.borrow().val);
        //         root = node.borrow_mut().right.take();
        //     }
        // }
        // ans

        // iter style
        use std::iter;
        fn inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Box<dyn iter::Iterator<Item = i32>> {
            match root {
                None => Box::new(iter::empty()),
                Some(rc) => {
                    let tree = rc.borrow();
                    Box::new(
                        inorder(&tree.left)
                            .chain(iter::once(tree.val))
                            .chain(inorder(&tree.right)),
                    )
                }
            }
        }

        inorder(&root).collect()
    }
}