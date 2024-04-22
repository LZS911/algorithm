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
type TreeRoot = Option<Rc<RefCell<TreeNode>>>;

pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let detect = |broken: &mut (TreeRoot, TreeRoot), prev: &mut TreeRoot, cur: &mut TreeRoot| {
        if prev.is_some()
            && prev.as_ref().unwrap().borrow().val > cur.as_ref().unwrap().borrow().val
        {
            if broken.0.is_none() {
                broken.0 = prev.clone();
            }
            broken.1 = cur.clone();
        }
    };

    if root.is_none() {
        return;
    }

    let mut broken = (None, None);
    let (mut prev, mut cur) = (None, root.clone());
    // 先左子树, 后右子树
    while cur.is_some() {
        if cur.as_ref().unwrap().borrow().left.is_none() {
            detect(&mut broken, &mut prev, &mut cur);
            prev = cur.clone();
            let tmp = cur.as_ref().unwrap().borrow().right.clone();
            cur = tmp;
        } else {
            let mut node = cur.as_ref().unwrap().borrow().left.clone();
            while node.as_ref().unwrap().borrow().right.is_some()
                && node.as_ref().unwrap().borrow().right != cur
            {
                let tmp = node.as_ref().unwrap().borrow().right.clone();
                node = tmp;
            }

            if node.as_ref().unwrap().borrow().right.is_none() {
                node.as_ref().unwrap().as_ref().borrow_mut().right = cur.clone();
                let tmp = cur.as_ref().unwrap().borrow().left.clone();
                cur = tmp;
            } else {
                detect(&mut broken, &mut prev, &mut cur);
                node.as_ref().unwrap().as_ref().borrow_mut().right = None;
                prev = cur.clone();
                let tmp = cur.as_ref().unwrap().borrow().right.clone();
                cur = tmp;
            }
        }
    }

    std::mem::swap(
        &mut broken.0.as_mut().unwrap().as_ref().borrow_mut().val,
        &mut broken.1.as_mut().unwrap().as_ref().borrow_mut().val,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {}
}
