use std::cell::RefCell;
use std::rc::Rc;

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

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }

    let node = root.as_ref().unwrap().borrow();

    dfs(&node.left, ans);

    ans.push(node.val);

    dfs(&node.right, ans);
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();

    dfs(&root, &mut ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {
        assert_eq!(inorder_traversal(None), vec![]);

        assert_eq!(
            inorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );

        assert_eq!(
            inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                })))
            })))),
            vec![1, 2, 3, 4]
        );
    }
}
