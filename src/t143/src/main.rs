impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = head.as_mut();
        while let Some(node) = cur.take() {
            if let Some(mut n) = node.next.as_ref() {
                if n.val == node.val {
                    node.next = node.next.take().unwrap().next;
                    cur = Some(node);
                    continue;
                } else {
                    cur = node.next.as_mut();
                }
            }
        }
        head
    }
}
