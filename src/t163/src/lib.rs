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

use std::cell::RefCell;
use std::rc::Rc;
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn recv(root: Option<Rc<RefCell<TreeNode>>>, pre: &mut [i64; 1]) -> bool {
        if let Some(node) = root {
            if !recv(node.borrow_mut().left.take(), pre) {
                return false;
            }
            if node.borrow().val as i64 <= pre[0] {
                return false;
            } else {
                pre[0] = node.borrow().val as i64;
            }
            recv(node.borrow_mut().right.take(), pre)
        } else {
            true
        }
    }
    let mut pre = [i64::MIN];
    recv(root, &mut pre)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {}
}
