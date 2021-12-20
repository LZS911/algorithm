import { node1, node2, node3, node4, node5 } from './../data/node.data';
import ListNode from '../utils/ListNode';

function deleteDuplicates(head: ListNode | null): ListNode | null {
  if (head?.val == undefined) {
    return null;
  }
  if (!head.next) {
    return head;
  }
  const res = head;

  while (head?.next) {
    if (head.val === head.next.val) {
      head.next = head.next.next;
    } else {
      head = head.next;
    }
  }
  return res;
}

deleteDuplicates(node1);
deleteDuplicates(node2);
deleteDuplicates(node3);
deleteDuplicates(node4);
deleteDuplicates(node5);
