// Definition for singly-linked list.
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let l1 = reverse_list(l1);
        let l2 = reverse_list(l2);

        reverse_list(carried(l1, l2, 0))
    }
}

fn carried(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carried(
                l1.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    carry += x.val;
                    x.next
                }),
                carry / 10,
            ),
            val: carry % 10,
        }))
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut head = head;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = pre;
        pre = Some(node);
    }

    pre
}
