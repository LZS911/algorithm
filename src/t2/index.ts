import { node1, node2, node3, node4, node5 } from '../data/list.node.data';
import ListNode from '../utils/ListNode';

function deleteDuplicates(head: ListNode | null): ListNode | null {
  if (head?.val == undefined) {
    return null;
  }
  if (!head.next) {
    return head;
  }
  const res = new ListNode(0, head);
  let prev = res;

  while (prev.next && prev.next.next) {
    if (prev.next.val === prev.next.next.val) {
      const num = prev.next.val;
      while (prev.next && prev.next.val === num) {
        prev.next = prev.next.next;
      }
    } else {
      prev = prev.next;
    }
  }

  return res.next;
}

deleteDuplicates(node1);
deleteDuplicates(node2);
deleteDuplicates(node3);
deleteDuplicates(node4);
deleteDuplicates(node5);
