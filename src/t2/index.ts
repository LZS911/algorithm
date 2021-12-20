import { node1, node2, node3, node4, node5 } from './../data/node.data';
import ListNode from '../utils/ListNode';

function deleteDuplicates(head: ListNode | null): ListNode | null {
  if (head?.val == undefined) {
    return null;
  }
  if (!head.next) {
    return head;
  }
  let next = head.next;
  let cur = head;

  while (next?.next) {
    while (cur.val !== next.val) {
      cur = next;
      next = next.next!;
    }
    cur.next = next.next!;
    next = next.next!;
  }

  return head;
}

// deleteDuplicates(node1);
deleteDuplicates(node2);
deleteDuplicates(node3);
deleteDuplicates(node4);
deleteDuplicates(node5);
