use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root.as_ref().unwrap().borrow().val {
            x @ (0 | 1) => x == 1,
            2 => {
                Solution::evaluate_tree(root.as_ref().unwrap().borrow().left.clone())
                    || Solution::evaluate_tree(root.as_ref().unwrap().borrow().right.clone())
            }
            _ => {
                Solution::evaluate_tree(root.as_ref().unwrap().borrow().left.clone())
                    && Solution::evaluate_tree(root.as_ref().unwrap().borrow().right.clone())
            }
        }
    }
}
