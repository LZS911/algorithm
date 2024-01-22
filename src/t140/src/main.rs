//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let mut prev = head?;
        let head = Self::insert_greatest_common_divisors(prev.next.take());
        if let Some(last) = head {
            let mut node = Box::new(ListNode::new(gcd(prev.val, last.val)));
            node.next = Some(last);
            prev.next = Some(node);
        }
        Some(prev)
    }
}
