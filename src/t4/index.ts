import { treeNode1 } from '../data/tree.node.data';
import TreeNode from '../utils/TreeNode';

function rangeSumBST(root: TreeNode | null, low: number, high: number): number {
  let sum = 0;
  const filter = (val: number) => val <= high && val >= low;
  const fn = (node: TreeNode | null) => {
    if (!node) {
      return sum;
    }
    if (filter(node.val)) {
      sum += node.val;
    }
    if (node.left) {
      fn(node.left);
    }
    if (node.right) {
      fn(node.right);
    }
    return sum;
  };

  return fn(root);
}

rangeSumBST(treeNode1, 7, 15);
