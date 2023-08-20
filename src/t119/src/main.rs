impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node1) = head {
            match node1.next.take() {
                None => return Some(node1),
                Some(mut node2) => {
                    node2.next.insert(node1).next = Self::swap_pairs(node2.next.take());
                    return Some(node2);
                }
            }
        }

        return None;
    }
}
