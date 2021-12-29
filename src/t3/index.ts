import ListNode from '../utils/ListNode';
import { node1, node2, node3, node4, node5 } from '../data/node.data';

function reverseList(head: ListNode | null): ListNode | null {
  if (!head) {
    return null;
  }
  if (!head.next) {
    return head;
  }
  let cur: ListNode | null = new ListNode(head.val, head.next);
  let prev = null;

  while (cur) {
    [cur.next, prev, cur] = [prev, cur, cur.next];
  }
  return prev;
}

reverseList(node1);
reverseList(node2);
reverseList(node3);
reverseList(node4);
reverseList(node5);
