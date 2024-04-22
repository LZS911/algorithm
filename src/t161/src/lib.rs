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

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    fn dfs(left: i32, right: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if left > right || right < left {
            vec![None]
        } else if left == right {
            vec![Some(Rc::new(RefCell::new(TreeNode::new(left))))]
        } else {
            let mut ans = vec![];
            for i in left..=right {
                let tl = dfs(left, i - 1);
                let tr = dfs(i + 1, right);
                for x in tl {
                    for y in tr.iter() {
                        ans.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: x.clone(),
                            right: y.clone(),
                        }))))
                    }
                }
            }
            ans
        }
    }
    dfs(1, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1() {}
}
